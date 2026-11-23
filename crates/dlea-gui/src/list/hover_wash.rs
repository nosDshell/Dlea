//! Settings > View > Highlight on hover as one window-wide value: off, no row, tile or rail row washes under the pointer.
use dlea::jsondoc::Json;
use gpui::{App, Global};

/// Ticked, the pointer's row, tile and rail row take the foreground wash and lift their ink.
/// A fresh document stores true. A missing or unreadable key stays off, so an old file that never wrote the key does not flip.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HoverWash(pub bool);

impl Global for HoverWash {}

impl HoverWash {
    /// Only a stored true ticks it. An absent or unreadable key stays off.
    pub fn read(data: &Json) -> Self {
        Self(data.get("highlightHover").and_then(Json::as_bool) == Some(true))
    }

    pub fn on(cx: &App) -> bool {
        cx.try_global::<Self>().is_some_and(|wash| wash.0)
    }

    /// Only a change reaches the global's observers.
    pub fn apply(data: &Json, cx: &mut App) {
        let wash = Self::read(data);
        if cx.try_global::<Self>().copied().unwrap_or_default() != wash {
            cx.set_global(wash);
        }
    }
}

#[cfg(test)]
#[path = "hover_wash_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "hover_surface_tests.rs"]
mod surface_tests;
