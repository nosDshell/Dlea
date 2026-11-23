//! The current address segment: a solid accent block, square on the left and rounded on the right.
//!
//! GXDE's crumb bar draws that block at a 4 px radius. The text uses the theme's light role
//! on the accent, which is the window ground in a light theme and the foreground in a dark one.

/// Room after a crumb that still has a neighbour, so the chevron is not glued to the next name.
pub const GAP: f32 = 4.;

/// Trailing pad inside the block, after the current name.
const TAIL: f32 = 6.;
/// The block starts this far before the name, inside the gap the previous chevron leaves.
const LEAD: f32 = 2.;
/// The block sits inside the address well, clear of its hairline.
const INSET: f32 = 4.;

pub const RADIUS: f32 = 4.;

pub struct Block {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// The placed width of one crumb. A neighbour keeps `GAP` after its chevron; the leaf stays on its glyphs.
pub fn placed_width(glyph: f32, last: bool) -> f32 {
    if last {
        glyph
    } else {
        glyph + GAP
    }
}

/// The current segment's block. `glyph_left` and `glyph_width` are the name's own box.
pub fn block(glyph_left: f32, glyph_width: f32, top: f32, band: f32) -> Block {
    Block {
        x: glyph_left - LEAD,
        y: top + INSET,
        width: glyph_width + LEAD + TAIL,
        height: (band - 2. * INSET).max(16.),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_current_block_is_square_on_the_left_of_the_name_and_padded_on_the_right() {
        let block = block(40., 48., 8., 30.);
        assert_eq!(block.x, 38.);
        assert_eq!(block.width, 56.);
        assert_eq!((block.y, block.height), (12., 22.));
        assert_eq!(placed_width(14., false), 18.);
        assert_eq!(placed_width(42., true), 42.);
    }
}
