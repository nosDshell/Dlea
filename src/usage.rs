use std::process::exit;

// Every invocation, as a usage error prints it to stderr and --help prints it to stdout.
const LINES: &[&str] = &[
    "usage: dlea [--tui|--gui] [--select <uri|path>] [path]",
    "       dlea --tui [--cwd-file PATH] [--chooser-file PATH] [--tui-keymap PRESET] [path]",
    "       dlea --default [off]",
    "       dlea --picker [off]",
    "       dlea --ui-state [<json patch>]",
    "       dlea --update [check]",
    "       dlea --run <path> <dev> <ino> <size> <mtime_s> <mtime_ns>",
    "       dlea --run-terminal <path> <dev> <ino> <size> <mtime_s> <mtime_ns>",
    "       dlea --open-text <path>",
    "       dlea --open-handler <path> <desktop id> <dev> <ino> <size> <mtime_s> <mtime_ns>",
    "       dlea --clip get|set copy|cut|clear TOKEN",
    "       dlea flash choose [start]",
    "       dlea plugin list [--json] | add <git url|folder> | enable|disable|update|remove <id> | validate <folder>",
    "       dlea --action <id> [--window <pid>] | --actions [--window <pid>]",
    "       dlea ipc [--pid N|--any] call <target> <function> [args...] | show",
    "       dlea --version | -V",
    "       dlea --help | -h",
];

pub(crate) fn usage(message: &str) -> ! {
    eprintln!("dlea: {}", message);
    for line in LINES {
        eprintln!("{line}");
    }
    exit(2)
}

// Asked for, so it goes to stdout and exits 0.
pub(crate) fn help() -> ! {
    for line in LINES {
        println!("{line}");
    }
    exit(0)
}
