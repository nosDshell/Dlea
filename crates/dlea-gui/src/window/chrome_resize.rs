//! The window's outer edges resize it. The title bar's buttons stay inside the inset.
use gpui::{Pixels, Point, ResizeEdge, Window};

const EDGE: f32 = 5.;

pub(crate) fn edge(window: &Window, at: Point<Pixels>) -> Option<ResizeEdge> {
    let size = window.viewport_size();
    let (x, y) = (f32::from(at.x), f32::from(at.y));
    let (width, height) = (f32::from(size.width), f32::from(size.height));
    if width < 1. || height < 1. {
        return None;
    }
    let left = x >= 0. && x < EDGE;
    let right = x <= width && x >= width - EDGE;
    let top = y >= 0. && y < EDGE;
    let bottom = y <= height && y >= height - EDGE;
    match (top, bottom, left, right) {
        (true, _, true, _) => Some(ResizeEdge::TopLeft),
        (true, _, _, true) => Some(ResizeEdge::TopRight),
        (_, true, true, _) => Some(ResizeEdge::BottomLeft),
        (_, true, _, true) => Some(ResizeEdge::BottomRight),
        (true, _, _, _) => Some(ResizeEdge::Top),
        (_, true, _, _) => Some(ResizeEdge::Bottom),
        (_, _, true, _) => Some(ResizeEdge::Left),
        (_, _, _, true) => Some(ResizeEdge::Right),
        _ => None,
    }
}
