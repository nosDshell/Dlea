//! Painted regression for HIG locked-menu, distinct from an enabled-menu prerequisite.
use super::*;
use dlea::jsondoc::Json;

fn number(value: &Json, key: &str) -> f64 {
    value.get(key).unwrap().as_f64().unwrap()
}

/// Glyphs of a centered count sit in the middle of the zone. A yielded count hugs its own text.
fn glyph_right(text: &Json) -> f64 {
    let (x, width, implicit) = (
        number(text, "x"),
        number(text, "width"),
        number(text, "implicitWidth"),
    );
    let drawn = implicit.min(width);
    if width > drawn + 2. {
        x + (width + drawn) / 2.
    } else {
        x + drawn
    }
}

fn clear_of(count: &Json, text: &Json) {
    assert!(
        number(text, "x") + 1. >= glyph_right(count) + 14. - 1.,
        "the notice covers the count: count={count:?} text={text:?}"
    );
}

#[gpui::test]
fn notice_layout_hidden_rows_keep_the_complete_hig_sentence(cx: &mut TestAppContext) {
    let handle = locked(cx, Some(super::HIDDEN_0_4_0));
    cx.simulate_window_resize(handle.into(), gpui::size(gpui::px(1100.), gpui::px(700.)));
    frame(cx, handle);
    let footer = json(cx, handle, "statusFooterState");
    println!(
        "locked-menu painted footer: {}",
        dlea::jsondoc::render_at(&footer, 0)
    );
    let primary = footer.get("centre").unwrap();
    let hint = footer.get("secondary").unwrap();
    // The bar is the 24 px status slot, not the 27 px list header. 700 - 24 = 676.
    assert_eq!(footer.get("frame").unwrap().as_str(), Some("0 676 1100 24"));
    handle
        .update(cx, |shell, _, _| {
            let face = shell.active_typeface();
            assert_eq!(f32::from(face.body), 14.);
            assert!(!shell.menu_opened());
        })
        .unwrap();
    assert_eq!(
        primary.get("text").unwrap().as_str(),
        Some("Open in Terminal, Permissions and Copy As are hidden in Settings > Menus.")
    );
    assert_eq!(hint.get("text").unwrap().as_str(), Some(" · esc dismisses"));
    assert_eq!(number(primary, "fontSize"), 12.);
    assert_eq!(
        primary.get("truncated").unwrap().as_bool(),
        Some(false),
        "HIG locked-menu requires the painted sentence in full: {primary:?}"
    );
    assert!(number(primary, "width") >= number(primary, "implicitWidth") - 1.);
    assert!(number(hint, "width") >= number(hint, "implicitWidth") - 1.);
    assert!(number(hint, "x") >= number(primary, "x") + number(primary, "width") - 1.);
    let left = footer.get("left").unwrap();
    let lane = footer.get("lane").unwrap();
    clear_of(left, primary);
    // The sentence ends at the slider, not against the undrawn disk string.
    assert!(
        (number(hint, "x") + number(hint, "width") - (number(lane, "x") + number(lane, "width")))
            .abs()
            <= 2.,
        "the hint should finish at the lane's slider edge: hint={hint:?} lane={lane:?}"
    );
    assert_eq!(read(cx, handle, "statusError"), Reply::value("false"));
    // The count stays the muted status ink. A non-error notice uses the foreground so it is not a second count, and the hint stays with the count.
    assert_ne!(primary.get("color"), left.get("color"));
    assert_eq!(hint.get("color"), left.get("color"));
}

#[gpui::test]
fn notice_layout_long_counts_and_utf8_errors_keep_their_own_space(cx: &mut TestAppContext) {
    let handle = open(cx);
    let sentence = format!(
        "Permission denied: /資料/{}résumé.txt",
        "long-folder/".repeat(40)
    );
    handle
        .update(cx, |shell, _, cx| {
            let pane = shell.panes[0].as_mut().unwrap();
            let held = &pane.pane.held;
            pane.pane.held = std::rc::Rc::new(crate::list::pane::Held {
                total: usize::MAX,
                listing: held.listing,
                start: held.start,
                rows: held.rows.clone(),
            });
            pane.selected = (0..1000).collect();
            shell.status_messages(cx, |messages, now| {
                messages.say(
                    &sentence,
                    true,
                    "",
                    dlea::ui_logic_status_feedback::status::Hint::None,
                    now,
                );
            });
        })
        .unwrap();
    for width in [600., 900.] {
        cx.simulate_window_resize(handle.into(), gpui::size(gpui::px(width), gpui::px(700.)));
        frame(cx, handle);
        let footer = json(cx, handle, "statusFooterState");
        println!(
            "long-count UTF-8 painted footer: {}",
            dlea::jsondoc::render_at(&footer, 0)
        );
        let left = footer.get("left").unwrap();
        let primary = footer.get("centre").unwrap();
        let hint = footer.get("secondary").unwrap();
        let lane = footer.get("lane").unwrap();
        assert!(
            number(left, "implicitWidth") > number(left, "width"),
            "count must exercise its cap"
        );
        assert!(number(lane, "x") >= number(left, "x") + number(left, "width") + 14. - 1.);
        clear_of(left, primary);
        assert_eq!(
            primary.get("text").unwrap().as_str(),
            Some(sentence.as_str())
        );
        assert_eq!(primary.get("truncated").unwrap().as_bool(), Some(true));
        assert_eq!(hint.get("text").unwrap().as_str(), Some(" · esc dismisses"));
        assert!(number(hint, "width") >= number(hint, "implicitWidth") - 1.);
        assert!(number(hint, "x") >= number(lane, "x") - 1.);
        assert!(
            number(hint, "x") + number(hint, "width")
                <= number(lane, "x") + number(lane, "width") + 1.
        );
        assert_ne!(primary.get("color"), left.get("color"));
        assert_eq!(read(cx, handle, "lastMessage"), Reply::value(&sentence));
        assert_eq!(read(cx, handle, "statusError"), Reply::value("true"));
    }
}

#[gpui::test]
fn notice_layout_localized_explanation_preserves_text_and_roles(cx: &mut TestAppContext) {
    let handle = open(cx);
    let sentence = "无法打开此文件夹。Les actions sont masquées dans Paramètres > Menus.";
    cx.simulate_window_resize(handle.into(), gpui::size(gpui::px(1100.), gpui::px(700.)));
    handle
        .update(cx, |shell, _, cx| {
            shell.status_messages(cx, |messages, now| {
                messages.explain(
                    sentence,
                    dlea::ui_logic_status_feedback::status::Hint::Dismiss,
                    now,
                );
            });
        })
        .unwrap();
    frame(cx, handle);
    let footer = json(cx, handle, "statusFooterState");
    let primary = footer.get("centre").unwrap();
    let hint = footer.get("secondary").unwrap();
    assert_eq!(primary.get("text").unwrap().as_str(), Some(sentence));
    assert_eq!(primary.get("truncated").unwrap().as_bool(), Some(false));
    let left = footer.get("left").unwrap();
    assert_ne!(primary.get("color"), left.get("color"));
    assert_eq!(hint.get("color"), left.get("color"));
    assert_eq!(read(cx, handle, "lastMessage"), Reply::value(sentence));
    assert_eq!(read(cx, handle, "statusError"), Reply::value("false"));
}
