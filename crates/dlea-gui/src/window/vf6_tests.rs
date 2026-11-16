#[test]
fn vf6_app_id_matches_desktop_and_picker() {
    let desktop = include_str!("../../../../packaging/com.nosDshell.dlea.desktop");
    assert!(desktop
        .lines()
        .any(|line| line == format!("StartupWMClass={}", super::APP_ID)));
    assert_eq!(super::APP_ID, "com.nosDshell.dlea");
    // The picker keeps v0.3.12's own id, the class every dlea --picker block in bindings.lua names.
    assert_eq!(
        crate::g2::picker_portal::launch::APP_ID,
        "com.nosDshell.dlea.picker"
    );
}
