//! HIG rail line boxes, including the extra group and the surfaces without ordinary chrome.
use super::*;
use crate::g2::rail_places::content::DrawnRow;
use flea::ui_logic_rail_places::rail::{Group, Kind};
use gpui::AppContext;

fn section(heading: &'static str, group: Group, labels: &[&str]) -> DrawnSection {
    DrawnSection {
        heading: heading.into(),
        caseless: false,
        rows: labels
            .iter()
            .map(|label| DrawnRow {
                label: label.to_string().into(),
                glyph: "house",
                owned: None,
                mark: RowMark::None,
                group,
                kind: Kind::Home,
                mounted: None,
                releasable: false,
                error: None,
                size: "".into(),
            })
            .collect(),
    }
}

#[test]
fn visual_geometry_rail_metrics_interpolate_and_then_scale() {
    for (base, top, gap, row, pad, horizontal_gap) in [
        (15, 9., 11., 32., 13., 8.),
        (16, 9., 11., 34., 14., 8.),
        (17, 9., 11., 36., 14., 9.),
        (18, 8., 10., 37., 14., 9.),
        (19, 8., 10., 39., 15., 9.),
        (20, 8., 10., 41., 15., 9.),
        (24, 10., 12., 49., 18., 11.),
        (28, 11., 14., 57., 21., 13.),
    ] {
        let metrics = RailMetrics::for_typeface(&Typeface::omarchy_default().at_base(base));
        assert_eq!(
            (metrics.pad, metrics.gap),
            (pad, horizontal_gap),
            "size {base}"
        );
        assert_eq!(
            (metrics.top, metrics.panel_gap, metrics.row),
            (top, gap, row),
            "size {base}"
        );
    }
    for base in [10, 12, 14] {
        let face = Typeface::omarchy_default().at_base(base);
        let list = crate::list::Metrics::for_typeface(&face);
        let metrics = RailMetrics::for_typeface(&face);
        assert_eq!(metrics.top, list.pad);
        assert_eq!((metrics.pad, metrics.gap), (PAD_X, GAP));
        assert_eq!(metrics.row, ROW);
        assert_eq!(metrics.icon, ICON);
        assert_eq!(
            metrics.panel_gap,
            (PANEL_GAP * base as f32 / OEM_BASE).round()
        );
    }
}

#[test]
fn visual_geometry_size20_rail_uses_the_compressed_board_bands() {
    let text_system = gpui_platform::current_platform(true).text_system();
    text_system
        .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
            "../../tests/fonts/JetBrainsMono-Regular.ttf"
        ))])
        .expect("the HIG font fixture loads");
    let mut cx = gpui::TestAppContext::build_with_text_system(
        gpui::TestDispatcher::new(0),
        None,
        text_system,
    );
    let handle = cx.add_window(|_, _| gpui::Empty);
    cx.update_window(handle.into(), |_, window, _| {
        let face = Typeface {
            font: gpui::font("JetBrains Mono"),
            ..Typeface::omarchy_default().at_base(20)
        };
        let sections = [
            section("PLACES", Group::Home, &["Home", "Trash"]),
            section("NETWORK", Group::Network, &["NAS"]),
            section("DEVICES", Group::Device, &["QEMU HARDDISK"]),
        ];
        let laid = layout(
            &sections,
            (274., 626.),
            &face,
            &face,
            &RowsModel::default(),
            window,
            &mut text::Cache::default(),
        );
        assert_eq!(laid.headers.len(), 2);
        assert_eq!(laid.rows.len(), 4);
        for header in &laid.headers {
            assert_eq!(header.rect.x, 15., "{} heading inset", header.text);
        }
        for (header, (top, height)) in laid.headers.iter().zip([(90., 24.), (155., 15.)])
        {
            assert_eq!(
                (header.rect.y, header.rect.height),
                (top, height),
                "{}",
                header.text
            );
        }
        for (row, top) in laid.rows.iter().zip([8., 49., 114., 170.]) {
            assert_eq!((row.rect.y, row.rect.height), (top, 41.), "{}", row.label);
            assert_eq!(row.label_rect.x, 46., "{} label inset", row.label);
        }
    })
    .unwrap();
}

#[test]
fn visual_geometry_rail_board_boxes_do_not_compress_later_groups() {
    let text_system = gpui_platform::current_platform(true).text_system();
    text_system
        .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
            "../../tests/fonts/JetBrainsMono-Regular.ttf"
        ))])
        .expect("the HIG font fixture loads");
    let mut cx = gpui::TestAppContext::build_with_text_system(
        gpui::TestDispatcher::new(0),
        None,
        text_system,
    );
    let handle = cx.add_window(|_, _| gpui::Empty);
    cx.update_window(handle.into(), |_, window, _| {
        let face = Typeface {
            font: gpui::font("JetBrains Mono"),
            ..Typeface::omarchy_default()
        };
        for favorites in [false, true] {
            let mut sections = vec![section("PLACES", Group::Home, &["Home", "Trash"])];
            if favorites {
                sections.push(section("FAVORITES", Group::Favorite, &["Work"]));
            }
            sections.extend([
                section("NETWORK", Group::Network, &["NAS"]),
                section("DEVICES", Group::Device, &["QEMU HARDDISK"]),
            ]);
            let laid = layout(
                &sections,
                (192., 673.),
                &face,
                &face,
                &RowsModel::default(),
                window,
                &mut text::Cache::default(),
            );
            let headers: Vec<(f32, f32)> = if favorites {
                vec![(74., 21.), (125., 30.), (185., 21.)]
            } else {
                vec![(74., 30.), (134., 21.)]
            };
            assert_eq!(laid.headers.len(), headers.len());
            for (header, (top, height)) in laid.headers.iter().zip(headers) {
                assert_eq!(header.rect.y, top, "{} heading top", header.text);
                assert_eq!(header.rect.height, height);
            }
            let rows = if favorites {
                vec![14., 44., 95., 155., 206.]
            } else {
                vec![14., 44., 104., 155.]
            };
            assert_eq!(laid.rows.len(), rows.len());
            for (row, top) in laid.rows.iter().zip(rows) {
                assert_eq!(row.rect.y, top, "{} row and menu anchor", row.label);
                assert_eq!(row.rect.height, 30.);
            }
            let home = &laid.rows[0];
            let run = text::Cache::default().get("Home", face.body, 150., 0., &face, window);
            assert!((home.label_rect.y - (14. + (30. - run.height) / 2. - 1.)).abs() < 0.001);
        }
    })
    .unwrap();
}
