use std::process::Command;

// The hand-off ui/TabBar.qml gives the one window a tear-off starts; nothing else Dlea starts may inherit it.
pub const ENV: [&str; 3] = ["DLEA_TAB_SOURCE_PID", "DLEA_TAB_CURSOR", "DLEA_TAB_TOKEN"];

// A terminal or an opened program that kept these would replay a finished lift into its own Dlea.
pub fn drop_env(command: &mut Command) {
    command.env_remove("DLEA_TAB_SORT");
    for name in ENV {
        command.env_remove(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Sample input: `"DLEA_TAB_TOKEN=" + root.outToken, ...` gives DLEA_TAB_TOKEN; only lines holding `on` are read.
    fn hand_off_names<'a>(source: &'a str, on: &str) -> Vec<&'a str> {
        let quoted = source
            .lines()
            .filter(|line| line.contains(on))
            .flat_map(|line| line.split('"'));
        quoted
            .filter(|word| word.starts_with("DLEA_TAB_"))
            .map(|word| word.trim_end_matches('='))
            .collect()
    }

    #[test]
    fn the_hand_off_names_exactly_what_the_tab_bar_sets() {
        assert_eq!(
            hand_off_names(include_str!("../ui/TabBar.qml"), "\"DLEA_TAB_"),
            ENV
        );
    }

    #[test]
    fn a_fresh_qml_launch_drops_the_same_names() {
        assert_eq!(
            hand_off_names(include_str!("../ui/js/Tabs.js"), "var TEAR_OFF_ENV"),
            ENV
        );
    }

    #[test]
    fn a_name_in_one_list_only_is_told_apart() {
        let grown =
            "execDetached([\"env\", \"DLEA_TAB_SOURCE_PID=\" + a, \"DLEA_TAB_EXTRA=\" + b])";
        assert_eq!(
            hand_off_names(grown, "\"DLEA_TAB_"),
            ["DLEA_TAB_SOURCE_PID", "DLEA_TAB_EXTRA"]
        );
        assert_ne!(hand_off_names(grown, "\"DLEA_TAB_"), ENV);
    }

    #[test]
    fn a_child_never_sees_a_variable_the_caller_set_for_it() {
        let mut child = Command::new("sh");
        child.args(["-c", "printf '%s' \"${DLEA_TAB_SOURCE_PID-}${DLEA_TAB_CURSOR-}${DLEA_TAB_TOKEN-}${DLEA_TAB_SORT-}\""]);
        for name in ENV {
            child.env(name, "stale");
        }
        child.env("DLEA_TAB_SORT", "stale");
        drop_env(&mut child);
        let out = child.output().unwrap();
        assert!(out.status.success());
        assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    }

    #[test]
    fn an_unrelated_variable_survives() {
        let mut child = Command::new("sh");
        child
            .args(["-c", "printf '%s' \"$DLEA_TAB_UNRELATED\""])
            .env("DLEA_TAB_UNRELATED", "kept");
        child.env("DLEA_TAB_SORT", "stale");
        drop_env(&mut child);
        assert_eq!(
            String::from_utf8_lossy(&child.output().unwrap().stdout),
            "kept"
        );
    }
}
