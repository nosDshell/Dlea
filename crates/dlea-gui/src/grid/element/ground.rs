//! Shared tile geometry and ground ink, including a chooser whose entry holds focus.
use super::{Drawn, GridState, Ink};
use crate::grid::geometry::Rect;
use gpui::{point, px, size, BorderStyle, Bounds, GlyphRun, Hsla, Pixels, TextRun, Window};

pub(super) fn caption_line<'a>(
    state: &'a GridState,
    drawn: &'a Drawn,
    at: usize,
) -> Option<&'a super::tile::Line> {
    let plain = drawn.tile.as_ref()?.lines.get(at)?;
    Some(
        state
            .bold
            .as_ref()
            .filter(|_| drawn.bold)
            .and_then(|bold| bold.lines[at].as_ref())
            .unwrap_or(plain),
    )
}

pub(in crate::grid::element) fn bounds_of(
    origin: gpui::Point<Pixels>,
    rect: Rect,
) -> Bounds<Pixels> {
    Bounds::new(
        point(origin.x + px(rect.x), origin.y + px(rect.y)),
        size(px(rect.width), px(rect.height)),
    )
}

pub(super) fn label_run(text: &str, state: &GridState, size: Pixels, window: &Window) -> GlyphRun {
    let run = TextRun {
        len: text.len(),
        font: state.font.clone(),
        color: Hsla::default(),
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    GlyphRun::new(window.text_system().layout_line(text, size, &[run], None))
}

// A run's baseline inside a fixed-height caption line, centred on the line box.
pub(super) fn baseline(run: &GlyphRun, top: Pixels, line: f32, window: &Window) -> Pixels {
    let layout = run.line();
    window.pixel_snap(top + (px(line) - layout.ascent - layout.descent) / 2. + layout.ascent)
}

// Both grid boards place the caption ink two pixels below the natural font top.
pub(super) fn caption_baseline(run: &GlyphRun, top: Pixels, window: &Window) -> Pixels {
    const TEXT_TOP: f32 = 2.;
    window.pixel_snap(top + px(TEXT_TOP) + run.line().ascent)
}

#[cfg(test)]
#[path = "caption_tests.rs"]
mod tests;

// Deepin 15 icon view (2017-11-24, Deepin Desktop 15.5) paints the selection on the
// filename only: a solid accent plate and light text, with the icon left unboxed.
const NAME_PAD_X: f32 = 6.;
const NAME_PAD_Y: f32 = 1.;
const NAME_RADIUS: f32 = 4.;

/// The filename plate, in grid pixels. Absent when the tile has no caption to sit it on.
pub(super) fn name_plate(drawn: &Drawn) -> Option<Rect> {
    if drawn.editing || !(drawn.cursor || drawn.selected) {
        return None;
    }
    if drawn.painted.width < 1. || drawn.painted.height < 1. {
        return None;
    }
    let x = (drawn.painted.x - NAME_PAD_X).max(drawn.cell.x + 1.);
    let right = (drawn.painted.x + drawn.painted.width + NAME_PAD_X)
        .min(drawn.cell.x + drawn.cell.width - 1.);
    let y = (drawn.painted.y - NAME_PAD_Y).max(drawn.cell.y);
    let bottom =
        (drawn.painted.y + drawn.painted.height + NAME_PAD_Y).min(drawn.cell.y + drawn.cell.height);
    Some(Rect {
        x,
        y,
        width: (right - x).max(1.),
        height: (bottom - y).max(1.),
    })
}

pub(super) fn caption_ink(drawn: &Drawn, focused: bool, ink: &Ink) -> Hsla {
    if focused && name_plate(drawn).is_some() {
        Hsla::white()
    } else {
        ink.foreground
    }
}

/// Filename plate when the tile has one, otherwise the hover or fallback tile box.
pub(super) fn paint_plate(
    window: &mut Window,
    frame: Bounds<Pixels>,
    origin: gpui::Point<Pixels>,
    drawn: &Drawn,
    ink: &Ink,
    focused: bool,
    selected: Option<Hsla>,
    corner: f32,
) {
    if let Some(rect) = name_plate(drawn) {
        let fill = if focused {
            ink.accent
        } else if drawn.selected {
            selected.unwrap_or_else(|| ink.accent.opacity(super::SEL_FILL))
        } else {
            ink.foreground.opacity(super::CURSOR_WASH)
        };
        window.paint_quad(gpui::quad(
            bounds_of(origin, rect),
            px(NAME_RADIUS.min(corner)),
            fill,
            px(0.),
            fill,
            BorderStyle::Solid,
        ));
        return;
    }
    if let Some(ground) = plate(drawn, ink, focused, selected) {
        let edged = focused && (drawn.cursor || drawn.selected);
        window.paint_quad(gpui::quad(
            frame,
            px(super::SEL_RADIUS.min(corner)),
            ground,
            px(if edged { 1. } else { 0. }),
            ink.accent.opacity(super::SEL_EDGE),
            BorderStyle::Solid,
        ));
    }
}

// The tile's own ground: the plate it sits on over the pane, which a cut picture fades toward.
pub(super) fn plate(
    drawn: &Drawn,
    ink: &Ink,
    focused: bool,
    selected: Option<Hsla>,
) -> Option<Hsla> {
    // A named selection is the filename plate, so the icon is not boxed or tinted.
    if name_plate(drawn).is_some() {
        return None;
    }
    // DStyledItemDelegate: cursor and selected tiles share the one accent selection box.
    if drawn.cursor && focused || drawn.selected && focused {
        Some(if drawn.cursor {
            ink.accent.opacity(super::SEL_FILL)
        } else {
            selected.unwrap_or_else(|| ink.accent.opacity(super::SEL_FILL))
        })
    } else if drawn.cursor {
        Some(ink.foreground.opacity(super::CURSOR_WASH))
    } else if drawn.selected {
        Some(ink.foreground.opacity(crate::list::TRACK))
    } else if drawn.hovered && ink.hover_wash {
        Some(ink.foreground.opacity(super::HOVER))
    } else {
        None
    }
}

impl Ink {
    /// The window's roles and Settings > View > Highlight on hover, read once per frame in prepaint.
    pub(super) fn for_window(window: &Window, cx: &gpui::App) -> Self {
        let theme = cx
            .try_global::<crate::palette::Theme>()
            .copied()
            .unwrap_or_default();
        Self {
            hover_wash: crate::list::hover_wash::HoverWash::on(cx),
            ..Self::inherited(&window.text_style(), theme)
        }
    }
}
