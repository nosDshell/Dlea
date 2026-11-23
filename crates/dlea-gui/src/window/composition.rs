//! Host geometry and typed producer slots; surfaces retain their own state and input handlers.
use super::startup::{Settings, View};
use gpui::{div, prelude::*, px, AnyElement, AnyView, Div, SharedString};

#[path = "composition_render.rs"]
mod composition_render;
pub(crate) use composition_render::render;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    fn at(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width: width.max(0.),
            height: height.max(0.),
        }
    }
    fn element(self) -> Div {
        div()
            .absolute()
            .left(px(self.x))
            .top(px(self.y))
            .w(px(self.width))
            .h(px(self.height))
            .overflow_hidden()
    }
}

#[derive(Clone, Debug)]
pub struct Geometry {
    pub chrome: Rect,
    pub tabs: Rect,
    pub rail: Rect,
    pub rail_overlay: bool,
    pub panes: [Rect; 2],
    pub status: Rect,
    /// The list header and the dual path row. The status bar is shorter, so it is not this band.
    list_band: f32,
}

impl Geometry {
    pub fn cover_single(&mut self) {
        self.chrome.height = 0.;
        self.tabs.height = 0.;
        self.rail.y = 0.;
        self.rail.height = self.status.y;
        self.panes[0].y = 0.;
        self.panes[0].height = self.status.y;
    }

    pub fn measure(width: f32, height: f32, settings: &Settings, view: View, tabs: bool) -> Self {
        Self::with_header(width, height, settings, view, tabs, 27.)
    }

    pub fn with_header(
        width: f32,
        height: f32,
        settings: &Settings,
        view: View,
        tabs: bool,
        header: f32,
    ) -> Self {
        Self::with_header_min(
            width,
            height,
            settings,
            view,
            tabs,
            header,
            crate::dual::ROOM_MIN,
        )
    }

    pub(crate) fn with_header_min(
        width: f32,
        height: f32,
        settings: &Settings,
        view: View,
        tabs: bool,
        header: f32,
        min: f32,
    ) -> Self {
        let width = width.max(0.);
        let height = height.max(0.);
        let chrome = 40.; // DDE 15 title bar. `header` is the list band.
                          // gxde-file-manager dstatusbar.cpp: inner widgets are 24 px and sizeHint is at least 22.
        let status_h = (header * 24. / 27.).round();
        let top = chrome + if tabs { header } else { 0. };
        let bottom = (height - status_h).max(top);
        let places = settings
            .data
            .get("places")
            .unwrap_or(&dlea::jsondoc::Json::Null);
        let mode = crate::rail::Mode::from_places(places);
        let rail = if mode == crate::rail::Mode::Docked {
            settings.rail_width.clamp(0., width)
        } else {
            0.
        };
        let room = (width - rail).max(0.);
        // DualRoom in WindowBody.qml: the room right of the rail less the hairline divider.
        let dual_room = (room - 1.).max(0.);
        let share = if view != View::Dual || dual_room < 2. * min {
            0.5
        } else {
            let low = min / dual_room;
            settings.split.clamp(low, 1. - low)
        };
        let left = if view == View::Dual {
            dual_room * share
        } else {
            room
        };
        Self {
            chrome: Rect::at(0., 0., width, chrome),
            tabs: Rect::at(0., chrome, width, top - chrome),
            rail: Rect::at(
                0.,
                top,
                if mode == crate::rail::Mode::Overlay {
                    width
                } else {
                    rail
                },
                bottom - top,
            ),
            rail_overlay: mode == crate::rail::Mode::Overlay,
            panes: [
                Rect::at(rail, top, left, bottom - top),
                Rect::at(
                    rail + left + 1.,
                    top,
                    (room - left - 1.).max(0.),
                    bottom - top,
                ),
            ],
            status: Rect::at(0., (height - status_h).max(0.), width, status_h),
            list_band: header,
        }
    }

    /// Where a pane's rows draw: below its path row, header and this pane's mounted strip.
    pub fn rows(&self, side: usize, dual: bool, strip: f32) -> Rect {
        let pane = self.panes[side];
        let band = self.list_band * u8::from(self.chrome.height > 0.) as f32;
        let top = band * (1. + if dual { 1. } else { 0. }) + strip;
        Rect::at(pane.x, pane.y + top, pane.width, pane.height - top)
    }

    pub fn rows_height(&self, dual: bool, strip: f32) -> f32 {
        let band = self.list_band * u8::from(self.chrome.height > 0.) as f32;
        (self.panes[0].height - band * (1. + if dual { 1. } else { 0. }) - strip).max(0.)
    }

    pub fn content_height(&self, view: View, strip: f32) -> f32 {
        self.content_height_for(0, view, strip)
    }

    pub fn content_height_for(&self, side: usize, view: View, strip: f32) -> f32 {
        if matches!(view, View::Columns | View::Grid) {
            (self.panes[side].height - strip).max(0.)
        } else {
            self.rows(side, view == View::Dual, strip).height
        }
    }
}

#[derive(Default)]
pub struct Slots {
    pub chrome: Option<AnyView>,
    pub rail: Option<AnyView>,
    pub tabs: Option<AnyView>,
    pub headers: [Option<AnyView>; 2],
    pub paths: [Option<AnyView>; 2],
    pub strips: [Option<AnyView>; 2],
    pub states: [Option<AnyView>; 2],
    pub covers: [Option<AnyView>; 2],
    pub strip_height: f32,
    pub status: Option<AnyView>,
}

impl Slots {
    pub(crate) fn strip_height_for(&self, side: usize) -> f32 {
        if self.strips[side].is_some() {
            self.strip_height
        } else {
            0.
        }
    }
}

#[derive(Clone)]
pub struct Labels {
    pub name: SharedString,
    pub size: SharedString,
    pub modified: SharedString,
}

pub(crate) struct Content<'a> {
    pub glass: super::glass::Grounds,
    pub paths: [SharedString; 2],
    pub headers: [Option<(AnyElement, f32)>; 2],
    pub status: SharedString,
    pub labels: &'a Labels,
    pub typeface: crate::list::Typeface,
    pub focus: &'a crate::dual::focus::Focus,
    pub pane_edge: Option<Div>,
}
