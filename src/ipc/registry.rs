use super::wire::Reply;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Function {
    pub target: &'static str,
    pub name: &'static str,
    pub parameters: &'static [(&'static str, &'static str)],
    pub result: &'static str,
}

macro_rules! function {
    ($target:literal, $name:literal, $result:literal, $parameters:expr) => {
        Function {
            target: $target,
            name: $name,
            result: $result,
            parameters: $parameters,
        }
    };
}

pub const ORACLE: &[Function] = &include!("functions.rs");
pub const LIFECYCLE: &[Function] = &[
    function!("dlea", "version", "string", &[]),
    function!("dlea", "quit", "void", &[]),
    function!("dleapicker", "version", "string", &[]),
    function!("dleapicker", "quit", "void", &[]),
];

pub const VISUAL: &[Function] = &[
    function!("dlea", "langmarksState", "string", &[]),
    function!("dlea", "langmarksCode", "string", &[("column", "bool")]),
    function!("dlea", "columnTrackRect", "string", &[("slot", "string")]),
    function!("dlea", "settingsRailWidth", "string", &[]),
    function!("dlea", "noMatchHeadline", "string", &[]),
    function!("dlea", "outlineState", "string", &[]),
    function!("dlea", "rowDepth", "int", &[("i", "int")]),
    function!("dlea", "rowExpanded", "bool", &[("i", "int")]),
    function!("dlea", "rowRelativePath", "string", &[("i", "int")]),
    function!("dlea", "omiState", "string", &[]),
    function!(
        "dlea",
        "scene",
        "string",
        &[
            ("window", "string"),
            ("name", "string"),
            ("settings", "string")
        ]
    ),
    function!(
        "dlea",
        "nativeGeometry",
        "string",
        &[
            ("window", "string"),
            ("scene", "string"),
            ("frame", "string")
        ]
    ),
    function!("dlea", "pendingRequests", "int", &[]),
    function!("dlea", "previewReadiness", "string", &[]),
    function!("dlea", "sceneState", "string", &[]),
    function!("dlea", "presentedFrame", "string", &[]),
    function!("dlea", "crumbCentreOf", "string", &[("label", "string")]),
    function!("dlea", "dragState", "string", &[]),
    function!("dlea", "railIconSize", "int", &[]),
    function!("dlea", "previewQuality", "string", &[]),
    // GPUI-only readers v0.3.12 never exposed over IPC: the Combine card, the pane's folder walks and their cancels, the Columns counters, the clipboard text and the tab tear-off.
    function!("dlea", "combineState", "string", &[]),
    function!("dlea", "dirSizeState", "string", &[]),
    function!("dlea", "columnsState", "string", &[]),
    function!("dlea", "columnAnimationState", "string", &[]),
    function!("dlea", "clipboardText", "string", &[]),
    function!("dlea", "dirSizeCancels", "int", &[]),
    function!("dlea", "tabTearoffState", "string", &[]),
    // The search strip's mode, query and laid-out box, and one Dual pane's list area, read by the ui.sh cases.
    function!("dlea", "searchMode", "string", &[]),
    function!("dlea", "searchQuery", "string", &[]),
    function!("dlea", "searchRect", "string", &[]),
    function!("dlea", "paneListAreaRect", "string", &[("side", "int")]),
];

/// The action entry point behind `dlea --action` and `dlea --actions`; see PLUGINS.md "Run an action from outside".
pub const ACTIONS: &[Function] = &[
    function!("dlea", "runAction", "string", &[("id", "string")]),
    function!("dlea", "actionIds", "string", &[]),
];

pub const CARRY: &[Function] = &[
    function!(
        "dlea",
        "carryOffer",
        "bool",
        &[("pid", "string"), ("token", "string")]
    ),
    function!("dlea", "carryPeek", "string", &[("token", "string")]),
    function!("dlea", "carryCancel", "bool", &[("token", "string")]),
    function!("dlea", "carryTaken", "bool", &[("token", "string")]),
    function!("dlea", "carryClaim", "bool", &[("token", "string")]),
];

pub fn functions() -> impl Iterator<Item = &'static Function> {
    ORACLE
        .iter()
        .chain(LIFECYCLE)
        .chain(VISUAL)
        .chain(ACTIONS)
        .chain(CARRY)
}

/// Quickshell's answer for a target with no seam loaded, which v0.3.12 gave until its first listing settled.
pub fn no_target() -> Reply {
    Reply::value("Target not found.")
}

pub fn validate(
    target: &str,
    name: &str,
    arguments: &[String],
) -> Result<&'static Function, Reply> {
    if !["dlea", "dleapicker", "dleatab"].contains(&target) {
        return Err(no_target());
    }
    let function = functions()
        .find(|f| f.target == target && f.name == name)
        .ok_or_else(|| Reply::value("Function not found."))?;
    if function.parameters.len() != arguments.len() {
        let amount = if arguments.len() < function.parameters.len() {
            "few"
        } else {
            "many"
        };
        return Err(Reply::value(format!("Too {amount} arguments provided ({} required but {} were provided.)\nFunction definition: {}",
            function.parameters.len(), arguments.len(), signature(function))));
    }
    for (index, ((_, kind), value)) in function.parameters.iter().zip(arguments).enumerate() {
        let valid = match *kind {
            "int" => value.trim().parse::<i32>().is_ok(),
            "bool" => ["true", "false", "1", "0"].contains(&value.as_str()),
            "string" => true,
            _ => false,
        };
        if !valid {
            return Err(Reply::value(format!("Unable to parse argument {} as \"{kind}\". Provided argument: \"{value}\"\nFunction definition: {}",
                index + 1, signature(function))));
        }
    }
    Ok(function)
}

fn signature(function: &Function) -> String {
    let parameters = function
        .parameters
        .iter()
        .map(|(name, kind)| format!("{name}: {kind}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "function {}({parameters}): {}",
        function.name, function.result
    )
}

pub fn refused(function: &Function) -> Reply {
    Reply::error(
        3,
        format!("DLEA_IPC_NOT_PORTED {}.{}", function.target, function.name),
    )
}

pub fn show() -> Reply {
    let mut output = String::new();
    for target in ["dlea", "dleapicker", "dleatab"] {
        output.push_str(&format!("target {target}\n"));
        for function in functions().filter(|f| f.target == target) {
            output.push_str(&format!("  {}\n", signature(function)));
        }
    }
    Reply {
        code: 0,
        stdout: output,
        stderr: String::new(),
    }
}

#[cfg(test)]
#[path = "registry_tests.rs"]
mod tests;
