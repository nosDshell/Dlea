//! The toolbar button owns pointer capture, its sized mark and its focus frame.
use super::plate::{self, Kind};
use super::{ChromeView, Sizes, ACTIONS, COMPUTER, COPY, FORWARD, GLYPHS, SIDEBAR};
use crate::list::Palette;
use crate::path::mark::{mark_ink, mark_sized, resting_ink};
use crate::window::startup::View;
use gpui::{canvas, div, prelude::*, px, Context, MouseButton};

impl ChromeView {
    pub(super) fn button(
        &self,
        sizes: Sizes,
        index: usize,
        enabled: bool,
        icon_on: bool,
        filled: bool,
        kind: Kind,
        palette: &Palette,
        ink_override: Option<gpui::Hsla>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let frame = self.frame.clone();
        let glyph = GLYPHS[index];
        {
            let mut frame = frame.borrow_mut();
            frame.controls[index].visible = true;
            frame.controls[index].reachable = true;
            frame.controls[index].enabled = enabled;
        }
        let focused = self.frame.borrow().controls[index].focused && enabled;
        let resting = ink_override.unwrap_or_else(|| resting_ink(enabled, icon_on, palette));
        let ink = mark_ink(enabled, focused, resting, palette.foreground);
        let (hovered, pressed) = {
            let control = self.frame.borrow().controls[index];
            (control.hovered, control.pressed)
        };
        // The copy mark draws nothing at rest. Search lives in the address well, so its
        // plate appears only while the pointer, keyboard or a press is on it.
        let show_plate = if index == COPY {
            resting.a > 0. || hovered || pressed || focused
        } else if index == 2 {
            hovered || pressed || focused
        } else {
            true
        };
        let recipe = plate::recipe(palette, kind, enabled, filled, hovered, pressed, show_plate);
        let plate_h = sizes.slot.min(sizes.band);
        let outer_h = if kind == Kind::Solo {
            sizes.band
        } else {
            plate_h
        };
        let plate_top = if kind == Kind::Solo {
            ((sizes.band - plate_h) / 2.).round()
        } else {
            0.
        };
        let hoverable = self.inputs.live;
        let live = enabled && hoverable;
        div()
            .id(("chrome-button", index))
            .w(px(sizes.slot))
            .h(px(outer_h))
            .flex_none()
            .cursor_pointer()
            .relative()
            .flex()
            .justify_center()
            .track_focus(&self.focus[index])
            .hover_listener_mode(gpui::HoverListenerMode::InputModalityIndependent)
            .on_hover(cx.listener(move |view, hovered: &bool, _, cx| {
                let mut frame = view.frame.borrow_mut();
                let control = &mut frame.controls[index];
                control.pointer_over = *hovered;
                control.hovered = control.pointer_over && control.enabled && view.inputs.live;
                let down = control.pointer_down && control.hovered && control.enabled;
                if control.pressed != down {
                    control.set_pressed(down, cx.background_executor().now(), cx.reduce_motion());
                    cx.notify();
                }
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |view, event: &gpui::MouseDownEvent, _, cx| {
                    if live && view.inputs.live {
                        let mut frame = view.frame.borrow_mut();
                        let control = &mut frame.controls[index];
                        control.pointer_down = true;
                        control.press_origin = Some(event.position);
                        control.press_at = Some(cx.background_executor().now());
                        control.set_pressed(
                            true,
                            cx.background_executor().now(),
                            cx.reduce_motion(),
                        );
                        drop(frame);
                        cx.notify();
                    }
                    cx.stop_propagation();
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |view, event: &gpui::MouseUpEvent, _, cx| {
                    let mut frame = view.frame.borrow_mut();
                    let control = &mut frame.controls[index];
                    let activate = live
                        && view.inputs.live
                        && control.pointer_down
                        && super::super::tap::within_threshold(
                            control.press_origin,
                            event.position,
                        )
                        && super::super::tap::short_press(
                            control.press_at,
                            cx.background_executor().now(),
                        )
                        && control.pressed
                        && control
                            .bounds
                            .is_some_and(|bounds| bounds.contains(&event.position));
                    control.pointer_down = false;
                    control.set_pressed(false, cx.background_executor().now(), cx.reduce_motion());
                    drop(frame);
                    cx.notify();
                    if activate {
                        cx.emit(ACTIONS[index]);
                    }
                    cx.stop_propagation();
                }),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(move |view, _, _, cx| {
                    let mut frame = view.frame.borrow_mut();
                    let control = &mut frame.controls[index];
                    if control.pointer_down {
                        control.pointer_down = false;
                        control.set_pressed(
                            false,
                            cx.background_executor().now(),
                            cx.reduce_motion(),
                        );
                        cx.notify();
                    }
                }),
            )
            .child(
                div()
                    .mt(px(plate_top))
                    .h(px(plate_h))
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(kind == Kind::Solo, |plate| plate.rounded(px(plate::RADIUS)))
                    .bg(recipe.fill)
                    .when(recipe.border.is_some(), |plate| {
                        plate
                            .border_1()
                            .border_color(recipe.border.expect("border ink"))
                    })
                    .child(mark_sized(
                        glyph,
                        sizes.mark,
                        ink,
                        self.frame.clone(),
                        index,
                    )),
            )
            .child(
                canvas(
                    move |bounds, _, _| frame.borrow_mut().controls[index].bounds = Some(bounds),
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .when(focused, |button| {
                button.child(
                    div()
                        .absolute()
                        .left(px(((sizes.slot - sizes.ring) / 2.).round()))
                        .top(px(plate_top + ((plate_h - sizes.ring) / 2.).round()))
                        .size(px(sizes.ring))
                        .rounded(px(plate::RADIUS))
                        .border_1()
                        .border_color(palette.foreground),
                )
            })
    }

    /// DToolBar: sidebar, a joined back/forward/up group, the crumb, search, the view tray, settings.
    pub(super) fn band(
        &mut self,
        theme: crate::palette::Theme,
        palette: &Palette,
        sizes: Sizes,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let light = plate::light(palette.background);
        let mut nav = div()
            .flex()
            .flex_none()
            .h(px(sizes.slot))
            .rounded(px(plate::RADIUS))
            .overflow_hidden();
        if light {
            nav = nav
                .bg(palette.background)
                .border_1()
                .border_color(palette.foreground.opacity(0.10));
        }
        nav = nav
            .child(self.button(
                sizes,
                0,
                self.inputs.back,
                true,
                false,
                Kind::Nav,
                palette,
                None,
                cx,
            ))
            .child(self.button(
                sizes,
                FORWARD,
                self.inputs.forward,
                true,
                false,
                Kind::Nav,
                palette,
                None,
                cx,
            ))
            .child(self.button(
                sizes,
                1,
                self.inputs.up,
                true,
                false,
                Kind::Nav,
                palette,
                None,
                cx,
            ));
        // Search sits in the address well except in Grid, where filter and sort take its place.
        let mut tools = div().h_full().flex().items_center().flex_none();
        if self.inputs.view == View::Grid {
            tools = tools
                .child(self.button(
                    sizes,
                    3,
                    self.inputs.filter,
                    true,
                    false,
                    Kind::Solo,
                    palette,
                    None,
                    cx,
                ))
                .child(div().w(px(6.)).flex_none())
                .child(self.button(
                    sizes,
                    4,
                    self.inputs.sort,
                    true,
                    false,
                    Kind::Solo,
                    palette,
                    None,
                    cx,
                ))
                .child(div().w(px(3.)).flex_none());
        }
        let mut tray = div()
            .flex()
            .flex_none()
            .items_center()
            .h(px(sizes.slot + 2.))
            .px(px(1.))
            .rounded(px(plate::RADIUS))
            .overflow_hidden()
            .bg(plate::tray_ground());
        for (index, mode) in [
            (5, View::List),
            (6, View::Columns),
            (7, View::Grid),
            (8, View::Dual),
        ] {
            let on = self.inputs.view == mode;
            tray =
                tray.child(self.button(sizes, index, true, on, on, Kind::View, palette, None, cx));
        }
        let frame = self.frame.clone();
        let pointer_frame = self.frame.clone();
        let toolbar = div()
            .size_full()
            .relative()
            .child(crate::window::glass::producers::backdrop(
                crate::window::glass::producers::Producer::Chrome,
                theme.chrome(),
            ))
            .child(
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .px(px(sizes.pad))
                    .child(self.button(
                        sizes,
                        SIDEBAR,
                        self.inputs.sidebar,
                        self.inputs.rail_shown,
                        self.inputs.rail_shown,
                        Kind::Solo,
                        palette,
                        None,
                        cx,
                    ))
                    .child(div().w(px(6.)).flex_none())
                    .child(nav)
                    .child(div().w(px(6.)).flex_none())
                    .child(self.button(
                        sizes,
                        COMPUTER,
                        true,
                        true,
                        self.path.read(cx).inputs.paths[self.path.read(cx).inputs.side].as_ref()
                            == "/",
                        Kind::Solo,
                        palette,
                        None,
                        cx,
                    ))
                    .child(div().w(px(10.)).flex_none())
                    .child(self.address(sizes, palette, cx))
                    .child(div().w(px(10.)).flex_none())
                    .child(tools)
                    .child(tray)
                    .child(div().w(px(3.)).flex_none())
                    .child(self.button(sizes, 9, true, true, false, Kind::Solo, palette, None, cx))
                    .child(div().w(px(8.)).flex_none())
                    .child(self.window_controls(sizes, palette, cx)),
            );
        for control in &mut self.frame.borrow_mut().controls {
            control.hovered =
                control.pointer_over && control.visible && control.enabled && self.inputs.live;
            if !control.visible || !control.enabled || !self.inputs.live {
                control.pointer_down = false;
                control.set_pressed(false, cx.background_executor().now(), cx.reduce_motion());
            }
        }
        toolbar
            .child(
                div()
                    .absolute()
                    .bottom_0()
                    .w_full()
                    .h(px(1.))
                    .bg(palette.foreground.opacity(0.05)),
            )
            .child(
                canvas(
                    move |bounds, _, _| frame.borrow_mut().bounds = Some(bounds),
                    move |_, _, window, _| {
                        super::super::tap::observe(pointer_frame.clone(), window)
                    },
                )
                .absolute()
                .size_full(),
            )
    }
}
