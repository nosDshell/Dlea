//! Minimize, maximize and close on the navigation bar.
//! The marks are original stroke icons, not Deepin or GXDE assets.
use super::{ChromeView, Sizes};
use crate::list::Palette;
use gpui::{div, prelude::*, px, Context, Hsla, MouseButton, SharedString, Window};

const MIN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 12h14"/></svg>"#;
const MAX: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 5h14v14H5z"/></svg>"#;
const CLOSE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 6l12 12 M18 6L6 18"/></svg>"#;

impl ChromeView {
    pub(super) fn window_controls(
        &self,
        sizes: Sizes,
        palette: &Palette,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let frame = self.frame.clone();
        let close_hot = frame.borrow().close_hovered;
        div()
            .relative()
            .flex()
            .items_center()
            .flex_none()
            .gap(px(2.))
            .child(
                gpui::canvas(
                    |_, _, _| {},
                    move |bounds, _, _, _| {
                        frame.borrow_mut().window_controls = Some(bounds);
                    },
                )
                .absolute()
                .size_full(),
            )
            .child(button(sizes, palette, "min", MIN, cx, |window| {
                window.minimize_window();
            }))
            .child(button(sizes, palette, "max", MAX, cx, |window| {
                window.zoom_window();
            }))
            .child(self.close_button(sizes, palette, close_hot, cx))
    }

    fn close_button(
        &self,
        sizes: Sizes,
        palette: &Palette,
        hot: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let ink = if hot {
            close_ink(palette)
        } else {
            palette.foreground
        };
        let fill = if hot {
            close_fill(palette.error)
        } else {
            palette.foreground.opacity(0.)
        };
        hit(sizes, "close", cx, |window| window.remove_window())
            .rounded(px(4.))
            .bg(fill)
            .hover_listener_mode(gpui::HoverListenerMode::InputModalityIndependent)
            .on_hover(cx.listener(|view, hovered: &bool, _, cx| {
                let mut frame = view.frame.borrow_mut();
                if frame.close_hovered != *hovered {
                    frame.close_hovered = *hovered;
                    drop(frame);
                    cx.notify();
                }
            }))
            .child(mark(sizes, "close", CLOSE, ink))
    }
}

/// The resting close mark is the theme foreground. On hover the plate is red and the mark is light.
fn close_ink(palette: &Palette) -> Hsla {
    if palette.background.l > 0.62 {
        palette.background
    } else {
        palette.foreground
    }
}

/// Keep the theme's error hue and lift a dull urgent colour so the plate reads as red.
fn close_fill(error: Hsla) -> Hsla {
    Hsla {
        s: error.s.max(0.74),
        l: 0.52,
        a: 1.,
        ..error
    }
}

fn button(
    sizes: Sizes,
    palette: &Palette,
    kind: &'static str,
    data: &'static [u8],
    cx: &mut Context<ChromeView>,
    act: fn(&mut Window),
) -> impl IntoElement {
    hit(sizes, kind, cx, act).child(mark(sizes, kind, data, palette.foreground))
}

fn hit(
    sizes: Sizes,
    kind: &'static str,
    cx: &mut Context<ChromeView>,
    act: fn(&mut Window),
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(SharedString::from(format!("chrome-window-{kind}")))
        .w(px(sizes.slot))
        .h(px(sizes.slot))
        .flex()
        .items_center()
        .justify_center()
        .flex_none()
        .cursor_pointer()
        .on_mouse_up(
            MouseButton::Left,
            cx.listener(move |_, _, window, cx| {
                act(window);
                cx.stop_propagation();
            }),
        )
}

fn mark(sizes: Sizes, kind: &'static str, data: &'static [u8], ink: Hsla) -> impl IntoElement {
    let key = SharedString::from(format!("dlea-chrome:win-{kind}"));
    gpui::canvas(
        |_, _, _| {},
        move |bounds, _, window, cx| {
            let _ = window.paint_svg(
                bounds,
                key.clone(),
                Some(data),
                gpui::TransformationMatrix::unit(),
                ink,
                cx,
            );
        },
    )
    .size(px(sizes.mark))
}
