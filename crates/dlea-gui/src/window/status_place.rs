//! Where the count and the notice sit so a long sentence does not cover the count.
//!
//! DDE 15 centers the count and keeps the scale slider at the right. Disk free space is not drawn.
//! A short notice uses the gap between that count and the slider. A sentence that does not fit
//! there moves the count to the left margin and takes the rest of the bar, up to the slider.
use crate::feedback::spinner::{GAP as SPINNER_GAP, OMI_GRID_PX};
use crate::g2::status_feedback::line::Drawn;
use dlea::ui_logic_status_feedback::status::SEP;

/// DStatusBar m_scaleSlider: 120 px, 20 px off the right edge.
pub(super) const SLIDER_W: f32 = 120.;
pub(super) const SLIDER_MARGIN: f32 = 20.;
pub(super) const GAP: f32 = 14.;

pub(super) struct Bands {
    pub counts_left: f32,
    pub counts_width: f32,
    pub center: bool,
    pub lane_right: f32,
    pub lane_width: f32,
}

/// The right inset shared by the notice lane and the undrawn disk probe: slider, its margin, and one gap.
pub(super) fn content_inset() -> f32 {
    SLIDER_MARGIN + SLIDER_W + GAP
}

pub(super) fn bands_for(
    footer: &super::Footer,
    measure: impl Fn(&str) -> f32,
    cx: &gpui::App,
) -> Bands {
    let drawn = footer.message_line().read(cx).drawn().clone();
    let count = measure(&footer.counts);
    bands(
        footer.width,
        footer.zone_width,
        count,
        notice(&drawn, &measure),
    )
}

pub(super) fn notice(drawn: &Drawn, measure: impl Fn(&str) -> f32) -> f32 {
    let mut width = if drawn.busy {
        OMI_GRID_PX + SPINNER_GAP
    } else {
        0.
    };
    for part in [&drawn.primary, &drawn.hint, &drawn.extra] {
        if !part.is_empty() {
            width += measure(part);
        }
    }
    if !drawn.undo.is_empty() {
        width += measure(SEP) + measure(&drawn.undo);
    }
    width
}

pub(super) fn bands(width: f32, zone: f32, count_natural: f32, notice: f32) -> Bands {
    let zone = zone.max(0.);
    let width = width.max(0.);
    let content_right = (width - content_inset()).max(GAP);
    let count_cap = count_natural.min(zone);
    let zone_left = ((width - zone) / 2.).max(GAP);
    let glyph_right = zone_left + (zone + count_cap) / 2.;
    let right_room = (content_right - glyph_right - GAP).max(0.);
    // Four pixels of slack so a measure that is slightly short still yields instead of clipping.
    let fits = notice <= right_room - 4. || notice <= 0.5;
    if fits {
        let lane_left = (glyph_right + GAP).min(content_right);
        return Bands {
            counts_left: zone_left,
            counts_width: zone,
            center: true,
            lane_right: width - content_right,
            lane_width: (content_right - lane_left).max(0.),
        };
    }
    let counts_width = if count_natural <= zone {
        count_natural + 1.
    } else {
        zone
    };
    let lane_left = (GAP + counts_width + GAP).min(content_right);
    Bands {
        counts_left: GAP,
        counts_width,
        center: false,
        lane_right: width - content_right,
        lane_width: (content_right - lane_left).max(0.),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measure(text: &str) -> f32 {
        text.chars().count() as f32 * 6.
    }

    #[test]
    fn a_short_notice_leaves_the_count_centered() {
        let bands = bands(1100., 357., 48., 100.);
        assert!(bands.center);
        assert!((bands.counts_left - 371.5).abs() < 0.1);
        assert_eq!(bands.counts_width, 357.);
        let lane_left = 1100. - bands.lane_right - bands.lane_width;
        let glyph_right = 371.5 + (357. + 48.) / 2.;
        assert!(lane_left + 0.1 >= glyph_right + GAP);
        assert!((1100. - bands.lane_right - (1100. - content_inset())).abs() < 0.1);
    }

    #[test]
    fn a_long_notice_moves_the_count_left_of_the_sentence() {
        let bands = bands(1100., 357., 48., 700.);
        assert!(!bands.center);
        assert_eq!(bands.counts_left, GAP);
        assert!((bands.counts_width - 49.).abs() < 0.1);
        let lane_left = 1100. - bands.lane_right - bands.lane_width;
        assert!((lane_left - (GAP + bands.counts_width + GAP)).abs() < 0.1);
        assert!(bands.lane_width + 0.1 >= 700.);
    }

    #[test]
    fn a_huge_count_stays_inside_its_zone_when_it_yields() {
        let bands = bands(600., 191., 900., 800.);
        assert!(!bands.center);
        assert_eq!(bands.counts_width, 191.);
        let lane_left = 600. - bands.lane_right - bands.lane_width;
        assert!(lane_left + 0.1 >= GAP + 191. + GAP);
    }

    #[test]
    fn an_empty_bar_still_centers() {
        let idle = bands(900., 291., 42., 0.);
        assert!(idle.center);
        assert!(notice(&Drawn::default(), measure) == 0.);
    }
}
