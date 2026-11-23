//! Columns and dual stay reachable from the toolbar menu. Icon and list stay the pair.
use super::{ChromeView, Sizes};
use crate::list::Palette;
use crate::window::startup::View;
use gpui::{deferred, div, prelude::*, px, Context, MouseButton};

/// Letters on the solid accent: the window ground in a light theme, the foreground in a dark one.
fn on_accent(palette: &Palette) -> gpui::Hsla {
    if super::plate::light(palette.background) {
        palette.background
    } else {
        palette.foreground
    }
}

const ROWS: [(&str, View); 4] = [
    ("settings.choice.view_grid", View::Grid),
    ("settings.choice.view_list", View::List),
    ("settings.choice.view_columns", View::Columns),
    ("settings.choice.view_dual", View::Dual),
];

pub(super) fn button(
    view: &ChromeView,
    sizes: Sizes,
    palette: &Palette,
    cx: &mut Context<ChromeView>,
) -> impl IntoElement {
    let open = view.frame.borrow().view_menu;
    let extra = matches!(view.inputs.view, View::Columns | View::Dual);
    let filled = open || extra;
    let ink = if filled {
        on_accent(palette)
    } else {
        palette.foreground
    };
    let ground = if filled {
        palette.accent
    } else {
        palette.background.opacity(0.)
    };
    let mut mark = div()
        .id("chrome-view-menu")
        .relative()
        .w(px(sizes.slot))
        .h(px(sizes.slot))
        .flex_none()
        .cursor_pointer()
        .child(
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(ground)
                .text_size(px(11.))
                .text_color(ink)
                .child("▾"),
        )
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_mouse_up(
            MouseButton::Left,
            cx.listener(|view, _, _, cx| {
                let mut frame = view.frame.borrow_mut();
                frame.view_menu = !frame.view_menu;
                drop(frame);
                cx.notify();
                cx.stop_propagation();
            }),
        );
    if open {
        // deferred draws after the chrome's clip, the same way the path popup leaves the bar.
        mark = mark.child(deferred(menu(view, sizes, palette, cx)).with_priority(1));
    }
    mark
}

fn action(mode: View) -> &'static str {
    match mode {
        View::Grid => "viewGrid",
        View::List => "viewList",
        View::Columns => "viewColumns",
        View::Dual => "viewDual",
    }
}

fn menu(
    view: &ChromeView,
    sizes: Sizes,
    palette: &Palette,
    cx: &mut Context<ChromeView>,
) -> gpui::Div {
    let current = view.inputs.view;
    let mut rows = div()
        .absolute()
        .top(px(sizes.slot))
        .right_0()
        .min_w(px(148.))
        .py(px(4.))
        .rounded(px(6.))
        .bg(palette.background)
        .border_1()
        .border_color(palette.muted_frame)
        .shadow_sm()
        .occlude()
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation());
    for (key, mode) in ROWS {
        let on = current == mode;
        let label = view.path.read(cx).catalog.tr(key);
        let text = if on {
            on_accent(palette)
        } else {
            palette.foreground
        };
        let fill = if on {
            palette.accent
        } else {
            palette.background.opacity(0.)
        };
        rows = rows.child(
            div()
                .id(key)
                .w_full()
                .px(px(10.))
                .py(px(4.))
                .cursor_pointer()
                .bg(fill)
                .text_color(text)
                .text_size(px(13.))
                .child(label)
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(move |view, _, _, cx| {
                        view.frame.borrow_mut().view_menu = false;
                        cx.emit(action(mode));
                        cx.notify();
                        cx.stop_propagation();
                    }),
                ),
        );
    }
    rows
}
