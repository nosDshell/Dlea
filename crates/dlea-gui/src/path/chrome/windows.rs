//! Minimize, maximize and close on the navigation bar.
//! The marks are original stroke icons, not Deepin or GXDE assets.
use super::{ChromeView, Sizes};
use crate::list::Palette;
use gpui::{div, prelude::*, px, Context, MouseButton, SharedString, Window};

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
        div()
            .flex()
            .items_center()
            .flex_none()
            .gap(px(2.))
            .child(button(sizes, palette, "min", MIN, cx, |window| {
                window.minimize_window();
            }))
            .child(button(sizes, palette, "max", MAX, cx, |window| {
                window.zoom_window();
            }))
            .child(button(sizes, palette, "close", CLOSE, cx, |window| {
                window.remove_window();
            }))
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
    let ink = palette.foreground;
    let side = sizes.mark;
    let key = SharedString::from(format!("dlea-chrome:win-{kind}"));
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
        .child(
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
            .size(px(side)),
        )
}
