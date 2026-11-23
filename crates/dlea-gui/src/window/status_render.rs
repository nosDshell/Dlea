//! The footer bar and its scale slider. Counts, notices and disk facts stay with the footer type.
use super::{counts, place, Footer, ThumbSlid};
use crate::list::{Metrics, Palette, Typeface};
use gpui::{div, prelude::*, px, Context, IntoElement, Render, SharedString, Window};

impl Footer {
    /// The stop a press inside the slider names: its fraction across the groove, snapped to THUMB_STOPS.
    fn slider_index(&self, x: gpui::Pixels) -> usize {
        let last = crate::grid::geometry::THUMB_STOPS.len() - 1;
        let norm = ((f32::from(x) - self.slider_left.get()) / place::SLIDER_W).clamp(0., 1.);
        ((norm * last as f32) + 0.5) as usize
    }

    /// DStatusBar m_scaleSlider: a horizontal groove with an accent fill and a circular handle, which drags to a thumbnail stop.
    fn slider(&self, height: f32, ink: &Palette, cx: &mut Context<Self>) -> impl IntoElement {
        let last = (crate::grid::geometry::THUMB_STOPS.len() - 1) as f32;
        let centre = ((self.thumb as f32 / last) * place::SLIDER_W).clamp(7., place::SLIDER_W - 7.);
        let left = self.slider_left.clone();
        div()
            .absolute()
            .right(px(place::SLIDER_MARGIN))
            .top_0()
            .h(px(height))
            .w(px(place::SLIDER_W))
            .flex()
            .items_center()
            .cursor_pointer()
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|view, event: &gpui::MouseDownEvent, window, cx| {
                    // The strip keeps the keyboard; the press only moves the handle.
                    window.prevent_default();
                    view.slider_drag = true;
                    let index = view.slider_index(event.position.x);
                    if index != view.thumb {
                        cx.emit(ThumbSlid(index));
                    }
                    cx.notify();
                }),
            )
            .on_mouse_move(cx.listener(|view, event: &gpui::MouseMoveEvent, _, cx| {
                if view.slider_drag {
                    let index = view.slider_index(event.position.x);
                    if index != view.thumb {
                        cx.emit(ThumbSlid(index));
                    }
                }
            }))
            .on_mouse_up(
                gpui::MouseButton::Left,
                cx.listener(|view, _, _, _| view.slider_drag = false),
            )
            .on_mouse_up_out(
                gpui::MouseButton::Left,
                cx.listener(|view, _, _, _| view.slider_drag = false),
            )
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(4.))
                    .rounded_full()
                    .bg(ink.foreground.opacity(0.18))
                    .child(
                        gpui::canvas(
                            move |bounds, _, _| left.set(f32::from(bounds.origin.x)),
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .size_full(),
                    )
                    .child(
                        div()
                            .absolute()
                            .left_0()
                            .top_0()
                            .h_full()
                            .w(px(centre))
                            .rounded_full()
                            .bg(ink.accent),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(-5.))
                            .left(px(centre - 7.))
                            .size(px(14.))
                            .rounded_full()
                            .bg(if ink.background.l > 0.62 {
                                ink.background
                            } else {
                                ink.foreground
                            })
                            .border_1()
                            .border_color(ink.muted_frame),
                    ),
            )
    }
}

impl Render for Footer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.painted.clear();
        let theme = crate::window::glass::producers::chrome_theme(window, cx);
        let ink = Palette::inherited(&window.text_style(), theme);
        let typeface = Typeface::for_window(window, cx);
        let height = (Metrics::for_typeface(&typeface).header * 24. / 27.).round();
        let style = gpui::TextStyle {
            font_family: typeface.font.family.clone(),
            font_size: typeface.caption.into(),
            ..window.text_style()
        };
        let measure = |text: &str| {
            let width = window
                .text_system()
                .layout_line(text, typeface.caption, &[style.to_run(text.len())], None)
                .width;
            f32::from(width)
        };
        let bands = place::bands_for(self, &measure, cx);
        let counts_text = counts::elided(&self.counts, bands.counts_width, &measure);
        self.counts_text = counts_text.layout().clone();
        // A block zone, not a flex row: only a text given its zone's width is cut with the ellipsis StatusBar.qml draws.
        let text = |name, value: SharedString, child: gpui::AnyElement, wide: f32| {
            let element = self.painted.watch(
                div()
                    .relative()
                    .w(px(wide))
                    .h(px(height))
                    .line_height(px(height))
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .text_ellipsis()
                    .child(child),
                name,
                value.clone(),
            );
            if name == "counts" {
                crate::visual::geometry::label(
                    element,
                    "status-text".into(),
                    "status".into(),
                    value,
                    0.,
                    false,
                    self.probe.clone(),
                )
            } else {
                element
            }
        };
        let root = div()
            .relative()
            .w_full()
            .h(px(height))
            .flex_none()
            .border(self.border_width)
            // HIG list and rail-favorites: the status bar stands on --surface under its --hair edge.
            .child(super::super::glass::producers::status_backdrop(
                theme.surface,
            ))
            .font_family(typeface.font.family)
            .text_size(typeface.caption)
            .text_color(ink.muted)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .w_full()
                    .h(px(1.))
                    .bg(ink.foreground.opacity(0.12)),
            )
            .child(
                text(
                    "counts",
                    self.counts.clone(),
                    counts_text.into_any_element(),
                    bands.counts_width,
                )
                .absolute()
                .left(px(bands.counts_left))
                .when(bands.center, |count| count.text_center()),
            )
            .child(
                text(
                    "disk",
                    self.filesystem.clone(),
                    self.filesystem.clone().into_any_element(),
                    self.zone_width,
                )
                .absolute()
                .right(px(place::content_inset()))
                .text_right()
                .opacity(0.),
            )
            .child(
                self.painted.watch(
                    div()
                        .absolute()
                        .right(px(bands.lane_right))
                        .w(px(bands.lane_width))
                        .h(px(height))
                        .child(self.message_line()),
                    "lane",
                    SharedString::default(),
                ),
            )
            .child(self.slider(height, &ink, cx));
        let footer = div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.painted.watch(root, "frame", SharedString::default()))
            .children(
                self.message_line()
                    .read_with(cx, |line, cx| line.detail(window, cx)),
            );
        #[cfg(test)]
        let footer = super::paint_tests::watch(footer, self, cx);
        footer
    }
}
