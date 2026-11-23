//! Original folder and disk marks for the Computer place. Not Deepin or GXDE assets.
use gpui::{point, px, size, Bounds, Hsla, Pixels, SharedString, Window};

// A light folder with a darker flap and a stronger emblem. The shapes are original.
const LIP: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect x="6" y="18" width="52" height="38" rx="6" fill="currentColor"/></svg>"#;
const TAB: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><path fill="currentColor" d="M8 22h22l-4-7a4 4 0 0 0-3.6-2H12a4 4 0 0 0-4 4v5z"/></svg>"#;
const FOLDER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect x="6" y="18" width="52" height="32" rx="6" fill="currentColor"/></svg>"#;
// A tall drive: pale chassis, bright face, light seam, lamp and vents.
const CHASSIS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect x="12" y="6" width="40" height="52" rx="5" fill="currentColor"/></svg>"#;
const FACE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><path fill="currentColor" d="M17 10h30a5 5 0 0 1 5 5v23H12V15a5 5 0 0 1 5-5z"/></svg>"#;
const SEAM: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect x="12" y="38" width="40" height="3" fill="currentColor"/></svg>"#;
const LAMP: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><circle cx="22" cy="49" r="2.3" fill="currentColor"/><rect x="32" y="46.2" width="2.2" height="5.6" rx="0.6" fill="currentColor"/><rect x="37" y="46.2" width="2.2" height="5.6" rx="0.6" fill="currentColor"/><rect x="42" y="46.2" width="2.2" height="5.6" rx="0.6" fill="currentColor"/><rect x="47" y="46.2" width="2.2" height="5.6" rx="0.6" fill="currentColor"/></svg>"#;
const DESKTOP: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M4 5h16v11H4z M3 16h18v2H3z M9 18h6v2H9z"/></svg>"#;
const VIDEOS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M4 6h16v12H4z M6 8h2.2v2H6z M6 11h2.2v2H6z M6 14h2.2v2H6z M15.8 8H18v2h-2.2z M15.8 11H18v2h-2.2z M15.8 14H18v2h-2.2z M9.2 8h5.6v8H9.2z"/></svg>"#;
const MUSIC: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M16 3v10.2a3.4 3.4 0 1 1-2.2-3.2V6.2L9 7.4V14a3.4 3.4 0 1 1-2.2-3.2V6.2z"/></svg>"#;
const PICTURES: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><circle cx="8" cy="8" r="2.2" fill="currentColor"/><path fill="currentColor" d="M2 18l6-6 3.2 3.2L15 11l7 7z"/></svg>"#;
const DOCUMENTS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M6 2h8l5 5v15H6z M8 12h8v1.6H8z M8 15.2h8v1.6H8z M8 18.4h5v1.6H8z"/></svg>"#;
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
            "dlea-computer:chassis",
            CHASSIS,
            chassis(ink),
            cx,
        );
        paint_svg(
            window,
            bounds,
            "dlea-computer:face",
            FACE,
            drive_face(ink),
            cx,
        );
        paint_svg(window, bounds, "dlea-computer:seam", SEAM, mark_ink, cx);
        paint_svg(
            window,
            bounds,
            "dlea-computer:lamp",
            LAMP,
            drive_lamp(ink),
            cx,
        );
        return;
    }
    let body = folder_body(ink);
    paint_svg(
        window,
        bounds,
        "dlea-computer:lip",
        LIP,
        folder_shade(body),
        cx,
    );
    paint_svg(
        window,
        bounds,
        "dlea-computer:tab",
        TAB,
        folder_shade(body),
        cx,
    );
    paint_svg(window, bounds, "dlea-computer:folder", FOLDER, body, cx);
    let Some((mark, id)) = badge(label_key) else {
        return;
    };
    let (width, height) = (f32::from(bounds.size.width), f32::from(bounds.size.height));
    let side = width * 0.34;
    let mark_bounds = Bounds {
        origin: bounds.origin + point(px((width - side) / 2.), px(height * 0.38)),
        size: size(px(side), px(side)),
    };
    paint_svg(window, mark_bounds, id, mark, emblem(ink), cx);
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

/// A light sky face in the folder's hue, so the emblem can be the stronger color.
fn folder_body(ink: Hsla) -> Hsla {
    Hsla {
        s: (ink.s * 0.45 + 0.50).clamp(0.55, 0.95),
        l: (ink.l + 0.20).clamp(0.66, 0.76),
        ..ink
    }
}

/// The flap and the bottom edge, a step darker than the face.
fn folder_shade(body: Hsla) -> Hsla {
    Hsla {
        s: (body.s + 0.06).min(1.),
        l: (body.l - 0.10).clamp(0.50, 0.66),
        ..body
    }
}

/// The folder's own stronger hue, drawn on the light face.
fn emblem(ink: Hsla) -> Hsla {
    Hsla {
        s: ink.s.max(0.62),
        l: ink.l.clamp(0.42, 0.58),
        ..ink
    }
}

fn chassis(ink: Hsla) -> Hsla {
    Hsla {
        h: ink.h,
        s: 0.08,
        l: 0.86,
        a: ink.a,
    }
}

/// A brighter face in the same hue, so an olive executable ink still reads as a disk.
fn drive_face(ink: Hsla) -> Hsla {
    Hsla {
        s: ink.s.max(0.72),
        l: (ink.l + 0.12).clamp(0.48, 0.64),
        ..ink
    }
}

fn drive_lamp(ink: Hsla) -> Hsla {
    Hsla {
        s: ink.s.max(0.70),
        l: (ink.l - 0.04).clamp(0.38, 0.55),
        ..ink
    }
}
