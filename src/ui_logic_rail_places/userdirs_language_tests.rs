use super::user_dirs;
use crate::ui_logic_rail_places::places::{favorites, home_entries};

#[test]
fn only_standard_xdg_role_names_use_catalog_keys() {
    let dirs = concat!(
        "XDG_DOWNLOAD_DIR=\"$HOME/Downloads\"\n",
        "XDG_DOCUMENTS_DIR=\"$HOME/Office records\"\n",
        "XDG_MUSIC_DIR=\"$HOME/Documents\"\n",
        "XDG_UNKNOWN_DIR=\"$HOME/Desktop\"\n",
        "XDG_DESKTOP_DIR=\"$HOME\"\n",
    );
    let rows = user_dirs(dirs, "/fixture");
    assert_eq!(rows.len(), 3, "a disabled desktop and an unknown role add no place");
    assert_eq!(rows[2].label_key, "places.row.downloads");
    for (at, row) in rows.iter().enumerate() {
        if at != 2 {
            assert_eq!(row.label_key, "", "custom names are real names");
        }
    }
    let home = home_entries("/fixture", dirs, |_| "folder");
    assert_eq!(home[3].label_key, "places.row.downloads");
    assert_eq!(home[2].label, "Office records");
    let merged = favorites(
        "/fixture",
        dirs,
        "file:///saved/Downloads Downloads\nfile:///saved/Desktop Desktop",
        |_| "folder",
    );
    assert_eq!(merged[3].label_key, "places.row.downloads");
    for row in &merged[4..] {
        assert_eq!(
            row.label_key, "",
            "bookmark names never become product copy"
        );
    }
}

#[test]
fn every_gxde_place_role_uses_a_catalog_key() {
    // gxde-file-manager lists only these six roles; the rest of user-dirs.dirs is not a place.
    let dirs = concat!(
        "XDG_DESKTOP_DIR=\"$HOME/Desktop\"\n",
        "XDG_DOWNLOAD_DIR=\"$HOME/Downloads\"\n",
        "XDG_TEMPLATES_DIR=\"$HOME/Templates\"\n",
        "XDG_PUBLICSHARE_DIR=\"$HOME/Public\"\n",
        "XDG_DOCUMENTS_DIR=\"$HOME/Documents\"\n",
        "XDG_MUSIC_DIR=\"$HOME/Music\"\n",
        "XDG_PICTURES_DIR=\"$HOME/Pictures\"\n",
        "XDG_VIDEOS_DIR=\"$HOME/Videos\"\n",
        "XDG_PROJECTS_DIR=\"$HOME/Projects\"\n",
        "XDG_UNKNOWN_DIR=\"$HOME/Projects 2\"\n",
    );
    let keys: Vec<&str> = user_dirs(dirs, "/fixture")
        .iter()
        .map(|row| row.label_key)
        .collect();
    assert_eq!(
        keys,
        [
            "places.row.desktop",
            "places.row.videos",
            "places.row.music",
            "places.row.pictures",
            "places.row.documents",
            "places.row.downloads",
        ]
    );
    let renamed = user_dirs("XDG_DOCUMENTS_DIR=\"$HOME/Work\"\n", "/fixture");
    assert_eq!(
        (renamed[0].label.as_str(), renamed[0].label_key),
        ("Work", ""),
        "a renamed Documents folder keeps its name"
    );
}
