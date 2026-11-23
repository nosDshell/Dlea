//! Deepin 15's Computer place: the user's folders and the mounted disks, not a listing of `/`.
use crate::{list::Palette, state::Shell, window::navigate::Step};
use dlea::i18n::I18n;
use dlea::ui_logic_device_rail::{Kind, RailEntry};
use dlea::ui_logic_rail_places::order::COMPUTER;
use dlea::ui_logic_rail_places::userdirs::user_dirs;
use gpui::{
    canvas, div, prelude::*, px, AnyElement, Context, KeyDownEvent, MouseButton, MouseDownEvent,
    SharedString, Window,
};

#[path = "computer_marks.rs"]
mod marks;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Tile {
    pub path: String,
    pub label_key: &'static str,
    pub label: String,
    pub disk: bool,
    pub external: bool,
}

pub(crate) fn directory_tiles(body: &str, home: &str) -> Vec<Tile> {
    user_dirs(body, home)
        .into_iter()
        .filter(|dir| std::path::Path::new(&dir.path).is_dir())
        .map(|dir| Tile {
            path: dir.path,
            label_key: dir.label_key,
            label: dir.label,
            disk: false,
            external: false,
        })
        .collect()
}

/// Mounted volumes, plus the real root as System Disk when no device row already names it.
pub(crate) fn volume_tiles(entries: &[RailEntry]) -> (Vec<Tile>, Vec<Tile>) {
    let mut internal = Vec::new();
    let mut external = Vec::new();
    let mut saw_root = false;
    for entry in entries {
        if !matches!(entry.kind, Kind::Disk | Kind::Volume) || !entry.path.starts_with('/') {
            continue;
        }
        if entry.kind == Kind::Volume && !entry.mounted {
            continue;
        }
        if entry.path == "/" {
            if saw_root {
                continue;
            }
            saw_root = true;
        }
        let tile = Tile {
            path: entry.path.clone(),
            label_key: if entry.path == "/" {
                "places.row.system_disk"
            } else {
                ""
            },
            label: entry.label.clone(),
            disk: true,
            external: entry.removable && entry.kind == Kind::Volume,
        };
        if tile.external {
            external.push(tile);
        } else {
            internal.push(tile);
        }
    }
    if !saw_root {
        internal.insert(
            0,
            Tile {
                path: "/".into(),
                label_key: "places.row.system_disk",
                label: String::new(),
                disk: true,
                external: false,
            },
        );
    }
    (internal, external)
}

fn shown(host: &super::pane::PaneHost) -> bool {
    host.pane.place() == COMPUTER || host.path == COMPUTER
}

/// Wide enough that a 900 px window wraps My Directories after four folders, as the 15.5 page does.
const TILE: f32 = 160.;

fn label_of(tile: &Tile, catalog: &I18n) -> String {
    if tile.label_key.is_empty() {
        tile.label.clone()
    } else {
        catalog.tr(tile.label_key)
    }
}

impl Shell {
    pub(crate) fn computer_page(
        &mut self,
        side: usize,
        width: f32,
        palette: &Palette,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let active = self
            .panes
            .get(side)
            .and_then(Option::as_ref)
            .is_some_and(shown);
        let covered = (side == 1 && self.view != crate::window::startup::View::Dual)
            || self.slots.covers[side].is_some()
            || self.flash_covers(side);
        if !active || covered {
            if let Some(host) = self.panes.get_mut(side).and_then(Option::as_mut) {
                host.computer_items = None;
            }
            return None;
        }
        let home = std::env::var("HOME").unwrap_or_default();
        let folders = directory_tiles(&crate::g2::rail_places::producer::dirs_text(&home), &home);
        let devices = self
            .rail
            .as_ref()
            .map(|join| join.devices.read(cx).devices.entries.clone())
            .unwrap_or_default();
        let (internal, external) = volume_tiles(&devices);
        let mut tiles = folders.clone();
        tiles.extend(internal.iter().cloned());
        tiles.extend(external.iter().cloned());
        let columns = ((width - 36.).max(TILE) / TILE).floor().max(1.) as usize;
        let selected = self.chrome.view.as_ref().and_then(|chrome| {
            chrome
                .read(cx)
                .frame
                .borrow()
                .computer_selected
                .clone()
                .filter(|path| tiles.iter().any(|tile| &tile.path == path))
        });
        if let Some(chrome) = &self.chrome.view {
            let mut frame = chrome.read(cx).frame.borrow_mut();
            frame.computer_paths = tiles.iter().map(|tile| tile.path.clone()).collect();
            frame.computer_columns = columns;
            frame.computer_selected.clone_from(&selected);
        }
        if let Some(host) = self.panes.get_mut(side).and_then(Option::as_mut) {
            let contained = selected.as_deref().and_then(folder_children);
            host.computer_items = Some((tiles.len(), usize::from(selected.is_some()), contained));
            host.drawn_view = None;
            host.drawn_list = false;
        }
        let catalog = self.locale.catalog.clone();
        let mut page = div()
            .id(("computer-overview", side))
            .size_full()
            .overflow_y_scroll()
            .p(px(18.))
            .flex()
            .flex_col();
        if !folders.is_empty() {
            page = page
                .child(heading(&catalog.tr("computer.section.directories"), palette))
                .child(wrap(side, &folders, &selected, palette, &catalog, cx));
        }
        page = page
            .child(heading(&catalog.tr("computer.section.internal"), palette))
            .child(wrap(side, &internal, &selected, palette, &catalog, cx));
        if !external.is_empty() {
            page = page
                .child(heading(&catalog.tr("computer.section.external"), palette))
                .child(wrap(side, &external, &selected, palette, &catalog, cx));
        }
        Some(page.into_any_element())
    }

    /// Arrows move the Computer selection. Enter opens the selected folder or disk.
    pub(crate) fn computer_key(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.keyboard.context != dlea::keys::Context::Listing
            || event.keystroke.modifiers != gpui::Modifiers::none()
            || self.path_holds_keyboard(window, cx)
            || self.menu_opened()
            || self.rename.edit.is_some()
        {
            return false;
        }
        let side = self.focus_side;
        if self.trash_opened(cx)
            || self.flash_covers(side)
            || !self.panes.get(side).and_then(Option::as_ref).is_some_and(shown)
        {
            return false;
        }
        let key = event.keystroke.key.as_str();
        let motion = matches!(key, "left" | "right" | "up" | "down");
        let open = matches!(key, "enter" | "return");
        if !motion && !open {
            return false;
        }
        let Some(chrome) = self.chrome.view.clone() else {
            return motion || open;
        };
        let (paths, columns, selected) = {
            let frame = chrome.read(cx).frame.borrow();
            (
                frame.computer_paths.clone(),
                frame.computer_columns.max(1),
                frame.computer_selected.clone(),
            )
        };
        if paths.is_empty() {
            return true;
        }
        let current = selected
            .as_deref()
            .and_then(|path| paths.iter().position(|own| own == path));
        if open {
            if let Some(index) = current {
                let _ = self.step(side, Step::Open(paths[index].clone()), window, cx);
            }
            return true;
        }
        let last = paths.len() - 1;
        let next = match (current, key) {
            (None, _) => 0,
            (Some(index), "left") => index.saturating_sub(1),
            (Some(index), "right") => (index + 1).min(last),
            (Some(index), "up") => index.saturating_sub(columns),
            (Some(index), _) => (index + columns).min(last),
        };
        chrome.update(cx, |view, cx| {
            view.frame.borrow_mut().computer_selected = Some(paths[next].clone());
            cx.notify();
        });
        if let Some(host) = self.panes.get_mut(side).and_then(Option::as_mut) {
            host.computer_items = Some((paths.len(), 1, folder_children(&paths[next])));
        }
        cx.notify();
        true
    }
}

fn heading(text: &str, palette: &Palette) -> gpui::Div {
    div()
        .w_full()
        .pt(px(10.))
        .pb(px(4.))
        .px(px(8.))
        .text_size(px(13.))
        .text_color(palette.foreground.opacity(0.55))
        .child(SharedString::from(text.to_owned()))
}

fn wrap(
    side: usize,
    tiles: &[Tile],
    selected: &Option<String>,
    palette: &Palette,
    catalog: &I18n,
    cx: &mut Context<Shell>,
) -> gpui::Div {
    let mut row = div().flex().flex_wrap();
    for tile in tiles {
        row = row.child(cell(
            side,
            tile,
            &label_of(tile, catalog),
            selected.as_deref() == Some(&tile.path),
            palette,
            cx,
        ));
    }
    row
}

fn cell(
    side: usize,
    tile: &Tile,
    name: &str,
    on: bool,
    palette: &Palette,
    cx: &mut Context<Shell>,
) -> impl IntoElement {
    let path = tile.path.clone();
    // Tag order is red, orange, yellow, green. Orange is the removable volume; executable is the theme green.
    let ink = if tile.external {
        palette.tags[1]
    } else if tile.disk {
        palette.executable
    } else {
        palette.accent
    };
    let on_accent = palette.background;
    let text = if on { on_accent } else { palette.foreground };
    let label_key = tile.label_key;
    let disk = tile.disk;
    div()
        .id(SharedString::from(format!("computer-{side}-{path}")))
        .w(px(TILE))
        .h(px(108.))
        .flex_none()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(6.))
        .cursor_pointer()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |shell, event: &MouseDownEvent, window, cx| {
                if event.click_count >= 2 {
                    let _ = shell.step(side, Step::Open(path.clone()), window, cx);
                } else if let Some(chrome) = shell.chrome.view.clone() {
                    chrome.update(cx, |view, cx| {
                        view.frame.borrow_mut().computer_selected = Some(path.clone());
                        cx.notify();
                    });
                }
                cx.stop_propagation();
            }),
        )
        .on_mouse_down(MouseButton::Right, cx.listener(|_, _, _, cx| {
            cx.stop_propagation();
        }))
        .child(
            canvas(
                |_, _, _| {},
                move |bounds, _, window, cx| {
                    marks::paint(window, bounds, disk, label_key, ink, on_accent, cx);
                },
            )
            .size(px(56.)),
        )
        .child(
            div()
                .max_w(px(100.))
                .px(px(6.))
                .rounded(px(4.))
                .text_size(px(12.))
                .text_color(text)
                .when(on, |name| name.bg(palette.accent))
                .overflow_hidden()
                .whitespace_nowrap()
                .text_ellipsis()
                .child(SharedString::from(name.to_owned())),
        )
}

/// Children of one user folder. A volume path is not a folder, and a name starting with a dot is not counted.
fn folder_children(path: &str) -> Option<usize> {
    let home = std::env::var_os("HOME")?;
    folder_children_in(path, std::path::Path::new(&home))
}

fn folder_children_in(path: &str, home: &std::path::Path) -> Option<usize> {
    let file = std::path::Path::new(path);
    if file.parent() != Some(home) {
        return None;
    }
    let read = std::fs::read_dir(file).ok()?;
    Some(
        read.filter_map(Result::ok)
            .filter(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
            .count(),
    )
}

#[cfg(test)]
#[path = "computer_tests.rs"]
mod tests;
