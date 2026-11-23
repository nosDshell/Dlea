//! Original folder and disk marks for the Computer place. Not Deepin or GXDE assets.
use gpui::{point, px, size, Bounds, Hsla, Pixels, SharedString, Window};

// A tabbed folder and a wide drive. The shapes are original, not Deepin or GXDE art.
const FOLDER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><path fill="currentColor" d="M6 28h52c2 0 4 2 4 4v20c0 3-2 5-5 5H7c-3 0-5-2-5-5V32c0-2 2-4 4-4z"/></svg>"#;
const TAB: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><path fill="currentColor" d="M6 28V23c0-4 3-6 7-6h15c2 0 3 1 4 3l4 8H6z"/></svg>"#;
const DRIVE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect x="8" y="22" width="48" height="26" rx="5" fill="currentColor"/></svg>"#;
const FACE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect x="8" y="16" width="48" height="22" rx="5" fill="currentColor"/></svg>"#;
const GRILLE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round"><path d="M16 40h32 M16 43.5h32 M16 47h32"/></svg>"#;
const DESKTOP: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M3 4h18v12H3z M9 16h6v2H9z M7 18h10v2H7z"/></svg>"#;
const VIDEOS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" fill-rule="evenodd" d="M3 5h18v14H3z M5 7h2v2H5z M5 11h2v2H5z M5 15h2v2H5z M17 7h2v2h-2z M17 11h2v2h-2z M17 15h2v2h-2z"/></svg>"#;
const MUSIC: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M16 3v10.2a3.4 3.4 0 1 1-2.2-3.2V6.2L9 7.4V14a3.4 3.4 0 1 1-2.2-3.2V6.2z"/></svg>"#;
const PICTURES: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><circle cx="8" cy="8" r="2.2" fill="currentColor"/><path fill="currentColor" d="M2 18l6-6 3.2 3.2L15 11l7 7z"/></svg>"#;
const DOCUMENTS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" fill-rule="evenodd" d="M6 2h8l5 5v15H6z M8 12h8v1.5H8z M8 15h8v1.5H8z M8 18h5v1.5H8z"/></svg>"#;
const DOWNLOADS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M11 3h2v8h3l-4 5-4-5h3z M5 18h14v2H5z"/></svg>"#;

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
    if disk {
        paint_svg(
            window,
            bounds,
            "dlea-computer:drive",
            DRIVE,
            deepen(ink),
            cx,
        );
        paint_svg(
            window,
            bounds,
            "dlea-computer:drive-face",
            FACE,
            drive_face(ink),
            cx,
        );
        paint_svg(
            window,
            bounds,
            "dlea-computer:drive-grille",
            GRILLE,
            mark_ink,
            cx,
        );
    } else {
        paint_svg(window, bounds, "dlea-computer:folder", FOLDER, ink, cx);
        paint_svg(
            window,
            bounds,
            "dlea-computer:folder-tab",
            TAB,
            deepen(ink),
            cx,
        );
    }
    let Some((mark, id)) = badge(label_key) else {
        return;
    };
    let (width, height) = (f32::from(bounds.size.width), f32::from(bounds.size.height));
    let side = width * 0.36;
    let mark_bounds = Bounds {
        origin: bounds.origin + point(px((width - side) / 2.), px(height * 0.46)),
        size: size(px(side), px(side)),
    };
    paint_svg(window, mark_bounds, id, mark, mark_ink, cx);
}

fn paint_svg(
    window: &mut Window,
    bounds: Bounds<Pixels>,
    key: &str,
    data: &[u8],
    ink: Hsla,
    cx: &gpui::App,
) {
    let _ = window.paint_svg(
        bounds,
        SharedString::from(key),
        Some(data),
        gpui::TransformationMatrix::unit(),
        ink,
        cx,
    );
}

/// A darker tab or drive base, still the same hue.
fn deepen(ink: Hsla) -> Hsla {
    Hsla {
        l: (ink.l - 0.14).clamp(0.12, 0.9),
        s: (ink.s + 0.06).clamp(0.0, 1.0),
        ..ink
    }
}

/// A brighter face in the same hue, so an olive executable ink still reads as a disk.
fn drive_face(ink: Hsla) -> Hsla {
    Hsla {
        s: ink.s.max(0.62),
        l: (ink.l + 0.20).clamp(0.50, 0.68),
        ..ink
    }
}
