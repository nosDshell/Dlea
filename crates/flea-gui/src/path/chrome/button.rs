//! The toolbar button owns pointer capture, its sized mark and its focus frame.
use super::{ChromeView, Sizes, ACTIONS, GLYPHS};
use crate::path::mark::{mark_ink, mark_sized};
use gpui::{canvas, div, prelude::*, px, Context, MouseButton};

impl ChromeView {
    pub(super) fn button(
        &self,
        sizes: Sizes,
        index: usize,
        enabled: bool,
        resting: gpui::Hsla,
        foreground: gpui::Hsla,
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
        let ink = mark_ink(enabled, focused, resting, foreground);
        // The gxde recipe: a hovered control rests on foreground at 0.10, a pressed one at 0.15.
        let (hovered, pressed) = {
            let control = self.frame.borrow().controls[index];
            (control.hovered, control.pressed)
        };
        let plate = if pressed {
            foreground.opacity(0.15)
        } else if hovered {
            foreground.opacity(0.10)
        } else {
            foreground.opacity(0.)
        };
        let hoverable = self.inputs.live;
        let live = enabled && hoverable;
        div()
            .id(("chrome-button", index))
            .w(px(sizes.slot))
            .h(px(sizes.band))
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
                    .h(px(sizes.inner()))
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(self.inputs.corner / 2.))
                    .bg(plate)
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
                        .top(px(((sizes.inner() - sizes.ring) / 2.).round()))
                        .size(px(sizes.ring))
                        .rounded(px(self.inputs.corner / 2.))
                        .border_1()
                        .border_color(foreground),
                )
            })
    }
}
