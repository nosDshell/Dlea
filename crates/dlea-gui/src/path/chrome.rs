//! The native toolbar owns its painted controls and hosts the existing path surface.
use super::{
    mark::{press_scale, Press, MARK},
    view::PathView,
};
use crate::{
    list::{Metrics, Palette, Typeface},
    window::startup::View,
};
use gpui::{
    div, prelude::*, Bounds, Context, Entity, EventEmitter, FocusHandle, Pixels, Point, Window,
};
use std::{cell::RefCell, rc::Rc, time::Instant};

mod address;
mod button;
mod plate;
mod view_menu;
mod windows;

/// The copy mark after the path is the last control, so every earlier index keeps its place.
pub const COPY: usize = 10;

pub const GLYPHS: [&str; 14] = [
    "arrow-left",
    "arrow-up",
    "search",
    "filter",
    "sort",
    "list",
    "columns",
    "grid",
    "dual",
    "sliders",
    "copy",
    "sidebar",
    "arrow-right",
    "computer",
];
pub const ACTIONS: [&str; 14] = [
    "back",
    "up",
    "search",
    "filter",
    "sortNext",
    "viewList",
    "viewColumns",
    "viewGrid",
    "viewDual",
    "settings",
    "copyAddress",
    "sidebar",
    "forward",
    "computer",
];
/// The order the keyboard walks the controls, as they stand left to right: the copy mark follows the path.
/// DDE 15 order: sidebar, back, forward, up, computer, then the path's copy mark, then the right cluster.
pub const ORDER: [usize; 14] = [11, 0, 12, 1, COMPUTER, COPY, 2, 3, 4, 5, 6, 7, 8, 9];
/// The sidebar toggle and the forward step stand beside Back and Up the way gxde-file-manager's toolbar draws them.
pub const SIDEBAR: usize = 11;
pub const FORWARD: usize = 12;
/// The computer/places button after back and forward. Up stays in the nav group.
pub const COMPUTER: usize = 13;

#[derive(Clone, Copy, Debug, Default)]
pub struct Control {
    pub bounds: Option<Bounds<Pixels>>,
    #[cfg(test)]
    pub mark_bounds: Option<Bounds<Pixels>>,
    #[cfg(test)]
    pub mark_ink: Option<gpui::Hsla>,
    /// The sprite's drawn width in device pixels: its snapped bounds times its transform's scale.
    #[cfg(test)]
    pub mark_drawn: Option<f32>,
    pub visible: bool,
    /// Built this frame and so on the keyboard's walk; the copy mark stays reachable while it draws nothing.
    pub reachable: bool,
    pub enabled: bool,
    pub focused: bool,
    pub hovered: bool,
    pointer_over: bool,
    pub pressed: bool,
    /// The initiating left press owns the tap only until release or permanent cancellation.
    pub pointer_down: bool,
    pub press_origin: Option<Point<Pixels>>,
    pub press_at: Option<Instant>,
    pub press: Option<Press>,
}

impl Control {
    pub fn set_pressed(&mut self, down: bool, now: Instant, reduced: bool) {
        if !down {
            self.pointer_down = false;
            self.press_origin = None;
            self.press_at = None;
        }
        if self.pressed != down {
            let from = press_scale(self.press, now, reduced).0;
            self.press = Some(Press {
                down,
                since: now,
                from,
            });
            self.pressed = down;
        }
    }
}

/// Theme.hitMin: a chrome button's hit box is at least this wide.
const HIT_MIN: f32 = 24.;

/// ChromeBar.qml and ChromeButton.qml at a text size: the band, the mark, its slot and focus ring, gaps and padding.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sizes {
    pub band: f32,
    pub mark: f32,
    pub slot: f32,
    pub ring: f32,
    pub gap: f32,
    pub pad: f32,
}

impl Sizes {
    pub fn at(face: &Typeface) -> Self {
        let metrics = Metrics::for_typeface(face);
        let mark = (MARK * f32::from(face.body) / 14.).round();
        let slot = HIT_MIN.max(mark);
        // Theme.chromeControlInset; the ring keeps the mark's parity so the mark sits centred in it.
        let inset = (f32::from(face.caption) / 4.)
            .round()
            .max(2. * metrics.hairline);
        // The title bar is 40 px at body 14, the DDE 15 address-toolbar height, and scales with the face.
        let band = (40. * f32::from(face.base) / 14.)
            .round()
            .max(metrics.header);
        let room = slot.min(band - metrics.hairline - 2. * inset);
        Self {
            band,
            mark,
            slot,
            ring: room - (room - mark).rem_euclid(2.),
            gap: metrics.gap,
            pad: metrics.pad,
        }
    }

    /// Keep the mark's parity so centring never puts an odd-sized mark half a pixel high.
    pub fn inner(&self) -> f32 {
        self.band - (self.band - self.mark).rem_euclid(2.)
    }
}

#[test]
fn visual_geometry_toolbar_marks_keep_the_board_origin_at_14_and_20() {
    for (base, band, mark, top) in [(14, 40., 16., 12.), (20, 57., 23., 17.)] {
        let sizes = Sizes::at(&Typeface::omarchy_default().at_base(base));
        assert_eq!((sizes.band, sizes.mark), (band, mark));
        assert_eq!((sizes.inner() - sizes.mark) / 2., top);
    }
}

#[derive(Default)]
pub struct Frame {
    pub bounds: Option<Bounds<Pixels>>,
    pub controls: [Control; 14],
    /// The open Settings card, whose marks underneath are left undrawn.
    pub cover: crate::g2::settings_pages::cover::Cover,
    /// The pointer is over the path's own slot, which is what shows the copy mark.
    pub address_hovered: bool,
    pub address_bounds: Option<Bounds<Pixels>>,
    pub window_controls: Option<Bounds<Pixels>>,
    /// The pointer is over Close, which fills that control red.
    pub close_hovered: bool,
    pub computer_selected: Option<String>,
    pub computer_paths: Vec<String>,
    pub computer_columns: usize,
    /// Columns and dual stay off the icon/list pair and open from this menu.
    pub view_menu: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Inputs {
    pub view: View,
    pub back: bool,
    pub up: bool,
    pub forward: bool,
    /// The rail mode the sidebar toggle can change: Overlay answers no toggle, so its button rests dim.
    pub sidebar: bool,
    /// Whether the rail draws: the toggle wears the checked ink the way a gxde sidebar button does.
    pub rail_shown: bool,
    pub filter: bool,
    pub sort: bool,
    pub live: bool,
    pub corner: f32,
}

pub struct ChromeView {
    pub path: Entity<PathView>,
    pub inputs: Inputs,
    pub frame: Rc<RefCell<Frame>>,
    pub focus: [FocusHandle; 14],
}

impl EventEmitter<&'static str> for ChromeView {}

impl ChromeView {
    pub fn new(path: Entity<PathView>, inputs: Inputs, cx: &mut Context<Self>) -> Self {
        Self {
            path,
            inputs,
            frame: Default::default(),
            focus: std::array::from_fn(|_| cx.focus_handle()),
        }
    }
}

impl Render for ChromeView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = crate::window::glass::producers::chrome_theme(window, cx);
        let palette = Palette::inherited(&window.text_style(), theme);
        for control in &mut self.frame.borrow_mut().controls {
            if control.visible {
                control.bounds = None;
            }
            control.visible = false;
            control.reachable = false;
            control.enabled = false;
        }
        for (index, focus) in self.focus.iter().enumerate() {
            self.frame.borrow_mut().controls[index].focused = focus.is_focused(window);
        }
        let sizes = Sizes::at(&Typeface::for_window(window, cx));
        self.band(theme, &palette, sizes, cx)
    }
}

pub struct PanePath {
    pub path: Entity<PathView>,
    pub side: usize,
}

impl Render for PanePath {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let probe = self.path.read_with(cx, |view, _| view.probe.clone());
        super::probe::wrap(
            div()
                .size_full()
                .child(super::render::pane_strip(&self.path, self.side, cx)),
            self.path.clone(),
            1 + self.side,
            probe,
        )
    }
}
