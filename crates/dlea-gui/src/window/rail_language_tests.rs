//! The mounted rail follows the same locale snapshot that Settings and Omarchy publish.
use super::*;
use dlea::ui_logic_rail_places::{
    order::{Heading, RailSection},
    plugin::PLUGINS_HEADING,
    rail::{Group, RailEntry},
};
use std::{fs, path::PathBuf, time::Duration};

struct LanguageFile(PathBuf);

impl LanguageFile {
    fn new() -> Self {
        let made = std::process::Command::new("mktemp")
            .args(["-d", "/tmp/dlea-rail-language.XXXXXXXX"])
            .output()
            .unwrap();
        assert!(made.status.success());
        let root = PathBuf::from(String::from_utf8(made.stdout).unwrap().trim());
        fs::write(root.join(".dlea-test-sandbox"), "rail language").unwrap();
        fs::create_dir_all(root.join("default/i18n")).unwrap();
        fs::write(
            root.join("default/i18n/locales.json"),
            include_str!("../../../../src/i18n/fixtures/locales.json"),
        )
        .unwrap();
        fs::write(root.join("language"), "en").unwrap();
        for (code, network) in [("en", "Network"), ("de", "Netzwerk"), ("zh-CN", "网络")] {
            let directory = root.join("default/i18n").join(code);
            fs::create_dir_all(&directory).unwrap();
            fs::write(
                directory.join("keybindings.json"),
                format!(r#"{{"Network":"{network}"}}"#),
            )
            .unwrap();
        }
        Self(root)
    }
}

impl Drop for LanguageFile {
    fn drop(&mut self) {
        assert!(self.0.is_absolute() && self.0.starts_with("/tmp"));
        assert!(self.0.join(".dlea-test-sandbox").is_file());
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn snapshot(updates: &async_channel::Receiver<Snapshot>) -> Snapshot {
    let updates = updates.clone();
    let (send, receive) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = send.send(updates.recv_blocking());
    });
    receive
        .recv_timeout(Duration::from_secs(10))
        .expect("live catalog worker")
        .unwrap()
}

#[gpui::test]
fn runtime_language_switch_relabels_fixed_places(cx: &mut TestAppContext) {
    let scratch = LanguageFile::new();
    let (_service, updates) = crate::window::locale::Service::start(dlea::i18n::Paths {
        omarchy: scratch.0.clone(),
        language_file: scratch.0.join("language"),
        catalogs: vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../i18n")],
    })
    .unwrap();
    assert_eq!(snapshot(&updates).catalog.language(), "en");
    let (shell, cx) = open(false, "list", cx);
    shell.update(cx, |shell, cx| {
        let mut state = shell.launch.settings.data.clone();
        crate::g2::settings_pages::seams::merge(
            &mut state,
            &dlea::jsondoc::parse(r#"{"places":{"showRecent":true,"showTags":true,"favourites":[{"label":"Downloads","path":"/saved/Downloads"}]}}"#).unwrap(),
        );
        let producer = shell.rail.as_ref().unwrap().places.clone();
        producer.update(cx, |producer, cx| {
            producer.adopt_state(&state, cx);
            producer.fixture_home(
                "/fixture",
                concat!(
                    "XDG_DESKTOP_DIR=\"$HOME/Desktop\"\n",
                    "XDG_DOCUMENTS_DIR=\"$HOME/Documents\"\n",
                    "XDG_DOWNLOAD_DIR=\"$HOME/Downloads\"\n",
                    "XDG_MUSIC_DIR=\"$HOME/Music\"\n",
                    "XDG_PICTURES_DIR=\"$HOME/Pictures\"\n",
                    "XDG_VIDEOS_DIR=\"$HOME/Videos\"\n",
                    "XDG_TEMPLATES_DIR=\"$HOME/Templates\"\n",
                    "XDG_PUBLICSHARE_DIR=\"$HOME/Public\"\n",
                    "XDG_PROJECTS_DIR=\"$HOME/My projects\"\n",
                ),
                cx,
            );
            producer.tags_seen(&[true; 7], cx);
            producer.set_tail(
                vec![RailSection {
                    heading: Heading::Key("settings.places.network"),
                    rows: vec![RailEntry {
                        group: Group::Network,
                        kind: Kind::Share,
                        label: "NAS".into(),
                        ..Default::default()
                    }],
                }, RailSection {
                    heading: Heading::Key("settings.places.devices"),
                    rows: vec![RailEntry {
                        group: Group::Device,
                        kind: Kind::Volume,
                        label: "Documents".into(),
                        ..Default::default()
                    }],
                }, RailSection {
                    heading: Heading::Plugin("Geräte".into()),
                    rows: vec![RailEntry {
                        group: Group::Plugin,
                        kind: Kind::PluginRow,
                        label: "Phone".into(),
                        ..Default::default()
                    }],
                }],
                cx,
            );
        });
    });
    frame(cx);
    let producer = shell.read_with(cx, |shell, _| shell.rail.as_ref().unwrap().places.clone());
    for (revision, language, expected) in [
        (2, "zh-CN", ["桌面", "视频", "音乐", "图片", "文档", "下载"]),
        (
            3,
            "de",
            [
                "Schreibtisch",
                "Videos",
                "Musik",
                "Bilder",
                "Dokumente",
                "Downloads",
            ],
        ),
        (
            4,
            "en",
            [
                "Desktop",
                "Videos",
                "Music",
                "Pictures",
                "Documents",
                "Downloads",
            ],
        ),
    ] {
        fs::write(scratch.0.join("language"), language).unwrap();
        let translated = snapshot(&updates);
        let catalog = translated.catalog.clone();
        assert_eq!(catalog.language(), language);
        shell.update(cx, |shell, cx| {
            assert_eq!(translated.revision, revision);
            shell.apply_locale(translated, cx);
        });
        frame(cx);
        producer.read_with(cx, |producer, _| {
            let labels = producer.shown_entries();
            assert_eq!(labels[0].as_ref(), catalog.tr("places.row.recent"));
            assert_eq!(labels[1].as_ref(), catalog.tr("places.row.home"));
            for (index, expected) in expected.into_iter().enumerate() {
                assert_eq!(
                    labels[index + 2].as_ref(),
                    expected,
                    "{language}: fixed place {index}"
                );
            }
            assert_eq!(labels[8].as_ref(), catalog.tr("places.row.trash"));
            assert_eq!(labels[9].as_ref(), catalog.tr("places.row.computer"));
            assert_eq!(labels[10].as_ref(), "Downloads", "user bookmark name");
            for (index, name) in dlea::ui_logic_rail_places::tags::NAMES
                .into_iter()
                .enumerate()
            {
                assert_eq!(
                    labels[index + 11].as_ref(),
                    catalog.tr(dlea::ui_logic_rail_places::tags::label_key(name))
                );
            }
            assert_eq!(labels[18].as_ref(), "NAS", "share name");
            assert_eq!(labels[19].as_ref(), "Documents", "volume label");
            for (section, key) in producer.provider.drawn.iter().zip([
                "places.heading.places",
                "places.heading.favorites",
                "places.heading.tags",
                "settings.places.network",
                "settings.places.devices",
            ]) {
                assert_eq!(
                    section.heading,
                    crate::g2::rail_places::content::heading_text(&catalog, &Heading::Key(key))
                );
            }
            // The plugin's title is German's DEVICES word, so only German files its row under PLUGINS (PLUGINS.md, Rail headings).
            let plugin = producer.provider.drawn.last().unwrap();
            let heading = match language {
                "de" => Heading::Key(PLUGINS_HEADING),
                _ => Heading::Plugin("Geräte".into()),
            };
            assert_eq!(
                plugin.heading,
                crate::g2::rail_places::content::heading_text(&catalog, &heading),
                "{language}: the plugin row's heading"
            );
            assert_eq!(plugin.rows[0].label.as_ref(), "Phone");
            let painted = producer.laid.borrow();
            assert!(painted.painted);
            for row in &painted.rows {
                assert_eq!(row.label, labels[row.index].as_ref());
            }
        });
        shell.read_with(cx, |shell, _| {
            assert_eq!(shell.rail.as_ref().unwrap().places, producer)
        });
    }
    shell.update(cx, |shell, _| shell.stop_for_app_quit());
}
