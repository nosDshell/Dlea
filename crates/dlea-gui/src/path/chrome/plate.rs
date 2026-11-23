//! Toolbar plates from gxde-file-manager `DFileManagerWindow.theme`.
//!
//! Dark controls (QPushButton#backButton and friends): foreground at 15% at rest,
//! 30% hovered, 10% pressed, 6% disabled, 4 px radius. A checked control is the
//! solid accent (`#2ca7f8` in the GXDE dark theme).
//!
//! Light controls rest on the window ground with a 10% hairline. A checked plate
//! is the solid accent in both themes; the mark paints white on that fill.
use crate::list::Palette;
use gpui::Hsla;

/// DToolBar buttons and the crumb well use a 4 px corner.
pub const RADIUS: f32 = 4.;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Sidebar, search, filter, sort and settings: their own rounded plate.
    Solo,
    /// One cell of the joined back / forward / up group.
    Nav,
    /// One cell of the view-mode tray (`ContollerToolBar`).
    View,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Recipe {
    pub fill: Hsla,
    pub border: Option<Hsla>,
}

/// A light window ground, the same split the chrome ink tests use.
pub fn light(background: Hsla) -> bool {
    background.l > 0.62
}

/// The controller tray's `rgba(0, 0, 0, 0.1)` ground, shared by both themes.
pub fn tray_ground() -> Hsla {
    let mut ink: Hsla = gpui::rgb(0x000000).into();
    ink.a = 0.10;
    ink
}

pub fn recipe(
    palette: &Palette,
    kind: Kind,
    enabled: bool,
    filled: bool,
    hovered: bool,
    pressed: bool,
    show: bool,
) -> Recipe {
    if !show {
        return Recipe {
            fill: palette.foreground.opacity(0.),
            border: None,
        };
    }
    let light = light(palette.background);
    if filled && enabled {
        let fill = palette.accent;
        return Recipe { fill, border: None };
    }
    if light {
        let fill = if !enabled {
            palette.background
        } else if pressed {
            palette.accent.opacity(0.28)
        } else if hovered {
            palette.accent.opacity(0.16)
        } else if kind == Kind::Nav {
            palette.foreground.opacity(0.)
        } else {
            palette.background
        };
        let border = if kind == Kind::Solo && enabled && (hovered || pressed) {
            Some(palette.accent)
        } else if kind == Kind::Solo && enabled {
            Some(palette.foreground.opacity(0.10))
        } else {
            None
        };
        return Recipe { fill, border };
    }
    let fill = if !enabled {
        palette.foreground.opacity(0.06)
    } else if pressed {
        palette.foreground.opacity(0.10)
    } else if hovered {
        palette.foreground.opacity(0.30)
    } else {
        palette.foreground.opacity(0.15)
    };
    Recipe { fill, border: None }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::Theme;

    fn palette(text: &str) -> Palette {
        Palette::inherited(&gpui::TextStyle::default(), Theme::from_text(text))
    }

    #[test]
    fn dark_toolbar_washes_follow_the_gxde_theme_file() {
        let palette = palette("background='#202020'\nforeground='#ffffff'\naccent='#2ca7f8'");
        let rest = recipe(&palette, Kind::Solo, true, false, false, false, true);
        assert!((rest.fill.a - 0.15).abs() < 1e-3, "{:?}", rest.fill);
        assert!(rest.border.is_none());
        let hover = recipe(&palette, Kind::Solo, true, false, true, false, true);
        assert!((hover.fill.a - 0.30).abs() < 1e-3);
        let pressed = recipe(&palette, Kind::Nav, true, false, true, true, true);
        assert!((pressed.fill.a - 0.10).abs() < 1e-3);
        let off = recipe(&palette, Kind::Solo, false, false, false, false, true);
        assert!((off.fill.a - 0.06).abs() < 1e-3);
        let on = recipe(&palette, Kind::View, true, true, false, false, true);
        assert_eq!(on.fill, palette.accent);
        assert!((tray_ground().a - 0.10).abs() < 1e-3);
    }

    #[test]
    fn light_solo_rests_on_the_ground_with_a_hairline() {
        let palette = palette("background='#ffffff'\nforeground='#000000'\naccent='#2ca7f8'");
        assert!(light(palette.background));
        let rest = recipe(&palette, Kind::Solo, true, false, false, false, true);
        assert_eq!(rest.fill, palette.background);
        assert!(rest.border.is_some());
        let on = recipe(&palette, Kind::View, true, true, false, false, true);
        assert_eq!(on.fill, palette.accent);
        let hidden = recipe(&palette, Kind::Solo, true, false, false, false, false);
        assert_eq!(hidden.fill.a, 0.);
    }
}
