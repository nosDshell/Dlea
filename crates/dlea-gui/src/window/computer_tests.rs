use super::*;
use dlea::ui_logic_device_rail::{Kind, RailEntry};

#[test]
fn an_absent_root_row_still_opens_the_real_root() {
    let (internal, external) = volume_tiles(&[]);
    assert!(external.is_empty());
    assert_eq!(internal.len(), 1);
    assert_eq!(internal[0].path, "/");
    assert_eq!(internal[0].label_key, "places.row.system_disk");
}

#[test]
fn a_root_disk_is_system_disk_and_a_removable_volume_is_external() {
    let root = RailEntry {
        path: "/".into(),
        label: "nvme0n1p2".into(),
        kind: Kind::Disk,
        mounted: true,
        ..RailEntry::default()
    };
    let stick = RailEntry {
        path: "/media/stick".into(),
        label: "PHOTOS".into(),
        kind: Kind::Volume,
        mounted: true,
        removable: true,
        ..RailEntry::default()
    };
    let idle = RailEntry {
        path: "/media/idle".into(),
        kind: Kind::Volume,
        mounted: false,
        removable: true,
        ..RailEntry::default()
    };
    let (internal, external) = volume_tiles(&[root, stick, idle]);
    assert_eq!(internal.len(), 1);
    assert_eq!(internal[0].label_key, "places.row.system_disk");
    assert_eq!(external.len(), 1);
    assert_eq!(external[0].path, "/media/stick");
    assert_eq!(external[0].label, "PHOTOS");
}

#[test]
fn standard_folders_carry_a_mark_and_a_renamed_folder_does_not() {
    assert!(super::marks::badge("places.row.downloads").is_some());
    assert!(super::marks::badge("").is_none());
}
