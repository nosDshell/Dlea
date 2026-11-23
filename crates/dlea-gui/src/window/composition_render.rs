//! The window's stacked surfaces. Geometry stays with the host.
use super::super::startup::View;
use super::{Content, Geometry, Slots};
use gpui::{div, prelude::*, px, AnyElement, Div, SharedString};

/// ChromeBar.qml, the tab strip and the docked rail take the keyboard only from Tab, never from a press.
fn keep_the_keys(_: &gpui::MouseDownEvent, window: &mut gpui::Window, _: &mut gpui::App) {
    window.prevent_default();
}

pub(crate) fn render(
    geometry: &Geometry,
    slots: &Slots,
    panes: [Option<AnyElement>; 2],
    content: Content<'_>,
    view: View,
    probe: Option<crate::visual::Probe>,
) -> Div {
    let dual = view == View::Dual;
    use crate::visual::geometry::{label, surface};
    let Content {
        paths,
        glass,
        mut headers,
        status,
        labels,
        typeface,
        focus,
        pane_edge,
    } = content;
    let metrics = crate::list::Metrics::for_typeface(&typeface);
    let text =
        |div: Div, id: String, parent: &str, value: &SharedString, inset: f32, right: bool| {
            label(
                div,
                id,
                parent.into(),
                value.clone(),
                inset,
                right,
                probe.clone(),
            )
        };
    let chrome = surface(geometry.chrome.element(), "chrome", probe.clone())
        .when(glass.glassy && slots.chrome.is_none(), |div| {
            div.child(glass.chrome())
        })
        .track_focus(&focus.chrome)
        .on_any_mouse_down(keep_the_keys)
        .text_size(typeface.caption);
    let mut root = div().size_full().child(match &slots.chrome {
        Some(view) => chrome.child(view.clone()),
        None => text(
            chrome.px(px(metrics.pad)),
            "chrome-path".into(),
            "chrome",
            &paths[0],
            metrics.pad,
            false,
        )
        .child(paths[0].clone()),
    });
    if let Some(view) = &slots.tabs {
        root = root.child(
            geometry
                .tabs
                .element()
                .track_focus(&focus.strip)
                .on_any_mouse_down(keep_the_keys)
                .child(view.clone()),
        );
    }
    if let Some(view) = slots.rail.as_ref().filter(|_| !geometry.rail_overlay) {
        root = root.child(
            geometry
                .rail
                .element()
                .track_focus(&focus.rail)
                .on_any_mouse_down(keep_the_keys)
                .child(view.clone()),
        );
    }
    for (side, content) in panes.into_iter().enumerate() {
        if side == 1 && !dual {
            continue;
        }
        if let Some(cover) = &slots.covers[side] {
            root = root.child(geometry.panes[side].element().child(cover.clone()));
            continue;
        }
        let Some(content) = content else { continue };
        let computer = paths[side].as_ref() == "computer://";
        let mut pane = geometry.panes[side].element().flex().flex_col();
        if dual {
            let id = format!("path-{side}");
            let row = surface(div(), id.clone(), probe.clone())
                .children(glass.path_fallback(side, slots.paths[side].is_some()))
                .relative()
                .h(px(metrics.header))
                .flex_none()
                .text_size(typeface.caption);
            pane = pane.child(match &slots.paths[side] {
                Some(view) => row.child(view.clone()),
                None => text(
                    row.px(px(metrics.pad)),
                    format!("{id}-text"),
                    &id,
                    &paths[side],
                    metrics.pad,
                    false,
                )
                .child(paths[side].clone()),
            });
        }
        let id = format!("header-{side}");
        let header = surface(div(), id.clone(), probe.clone())
            .when(glass.glassy, |div| div.child(glass.header()))
            .relative()
            .h(px(headers[side]
                .as_ref()
                .map_or(metrics.header, |(_, height)| *height)))
            .flex_none();
        let cell = |div: Div, key: &str, value: &SharedString, right: bool| {
            text(div.relative(), format!("{id}-{key}"), &id, value, 0., right).child(value.clone())
        };
        if matches!(view, View::List | View::Dual) && !computer {
            pane = pane.child(match &slots.headers[side] {
                Some(view) => header.child(view.clone()),
                None if headers[side].is_some() => header.child(headers[side].take().unwrap().0),
                None => header
                    .flex()
                    .items_center()
                    .pl(px(metrics.pad))
                    .pr(px(2. * metrics.pad))
                    .text_size(typeface.caption)
                    .child(cell(div().flex_1(), "name", &labels.name, false))
                    .child(cell(
                        div()
                            .w(px(70. * f32::from(typeface.body_small) / 13.))
                            .text_right(),
                        "size",
                        &labels.size,
                        true,
                    ))
                    .child(cell(
                        div()
                            .w(px(134. * f32::from(typeface.body_small) / 13.))
                            .text_right(),
                        "modified",
                        &labels.modified,
                        true,
                    )),
            });
        } else if let Some((element, _)) = headers[side].take() {
            // HIG cols4-hero-search: Columns has no list header, so only a search strip arrives here, above the columns.
            pane = pane.child(header.child(element));
        }
        if let Some(view) = slots.strips[side].as_ref().filter(|_| !computer) {
            pane = pane.child(
                div()
                    .when(glass.glassy, |div| div.child(glass.chrome()))
                    .h(px(slots.strip_height))
                    .flex_none()
                    .child(view.clone()),
            );
        }
        root = root.child(pane.child(div().flex_1().min_h_0().overflow_hidden().child(content)));
    }
    root = root.children(pane_edge);
    if let Some(view) = slots.rail.as_ref().filter(|_| geometry.rail_overlay) {
        // PaneRail.qml: the overlay owns only its edge and body (the host hears those presses), so its stop hangs on a zero-size node.
        root = root.child(
            geometry
                .rail
                .element()
                .child(div().absolute().size_0().track_focus(&focus.rail))
                .child(view.clone()),
        );
    }
    let bar = surface(geometry.status.element(), "status", probe.clone())
        .when(glass.glassy && slots.status.is_none(), |div| {
            div.child(glass.status_backdrop())
        })
        .px(px(metrics.pad))
        .text_size(typeface.caption);
    root.child(match &slots.status {
        Some(view) => bar.px(px(0.)).child(view.clone()),
        None => text(
            bar,
            "status-text".into(),
            "status",
            &status,
            metrics.pad,
            false,
        )
        .child(status),
    })
}
