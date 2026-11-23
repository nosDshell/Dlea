//! Original folder and disk marks for the Computer place. Not Deepin or GXDE assets.
use gpui::{point, px, size, Bounds, Hsla, Pixels, SharedString, Window};

// A tabbed folder and a wide drive. The shapes are original, not Deepin or GXDE art.
const FOLDER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><path fill="currentColor" d="M8 26V22c0-4 3-6 7-6h11c2 0 3 1 4 3l3 5h18c4 0 7 3 7 7v19c0 4-3 7-7 7H15c-4 0-7-3-7-7z"/></svg>"#;
const DRIVE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect x="6" y="18" width="52" height="28" rx="6" fill="currentColor"/></svg>"#;
const SLOT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect x="14" y="34" width="24" height="4" rx="1.5" fill="currentColor"/><circle cx="46" cy="36" r="2.2" fill="currentColor"/></svg>"#;
const DESKTOP: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M3 5h18v12H3z M8 19h8v2H8z"/></svg>"#;
const VIDEOS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M8 6l11 6-11 6z"/></svg>"#;
const MUSIC: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M15 4v9.2A3.2 3.2 0 1 1 12.4 10V4z"/></svg>"#;
const PICTURES: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><circle cx="12" cy="12" r="5" fill="currentColor"/></svg>"#;
const DOCUMENTS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M5 6h14v2H5z M5 11h14v2H5z M5 16h9v2H5z"/></svg>"#;
const DOWNLOADS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M11 4h2v8h3l-4 6-4-6h3z"/></svg>"#;

pub(super) fn badge(label_key: &str) -> Option<(&'static [u8], &'static str)> {
    Some(match label_key {
        "places.row.desktop" => (DESKTOP, "dlea-computer:desktop"),
        "places.row.videos" => (VIDEOS, "dlea-computer:videos"),
        "places.row.music" => (MUSIC, "dlea-computer:music"),
        "places.row.pictures" => (PICTURES, "dlea-computer:pictures"),
        "places.row.documents" => (DOCUMENTS, "dlea-computer:documents"),
        "places.row.downloads" => (DOWNLOADS, "dlea-computer:downloads"),
        _ => return None,
    })
}

pub(super) fn paint(
    window: &mut Window,
    bounds: Bounds<Pixels>,
    disk: bool,
    label_key: &str,
    ink: Hsla,
    mark_ink: Hsla,
    cx: &gpui::App,
) {
    let (data, key): (&[u8], &str) = if disk {
        (DRIVE, "dlea-computer:drive")
    } else {
        (FOLDER, "dlea-computer:folder")
    };
    let _ = window.paint_svg(
        bounds,
        SharedString::from(key),
        Some(data),
        gpui::TransformationMatrix::unit(),
        ink,
        cx,
    );
    if disk {
        let _ = window.paint_svg(
            bounds,
            SharedString::from("dlea-computer:drive-slot"),
            Some(SLOT),
            gpui::TransformationMatrix::unit(),
            mark_ink,
            cx,
        );
    }
    let Some((mark, id)) = badge(label_key) else {
        return;
    };
    let (width, height) = (f32::from(bounds.size.width), f32::from(bounds.size.height));
    let side = width * 0.34;
    let mark_bounds = Bounds {
        origin: bounds.origin + point(px((width - side) / 2.), px(height * 0.46)),
        size: size(px(side), px(side)),
    };
    let _ = window.paint_svg(
        mark_bounds,
        SharedString::from(id),
        Some(mark),
        gpui::TransformationMatrix::unit(),
        mark_ink,
        cx,
    );
}
