//! Test-support path helpers. Compiled unconditionally — not #[cfg(test)] —
//! because bin-target unit tests and tests/ integration binaries link the
//! library without cfg(test) set.

use std::path::Path;

/// `"/usr/bin/x"` where the path exists (FHS systems, the packaged target),
/// the bare name resolved through PATH elsewhere (Guix, NixOS, minimal
/// containers). Preferring the absolute path keeps a PATH shadow from
/// redirecting the fixture on systems that have it.
pub fn tool(path: &str) -> String {
    if Path::new(path).exists() {
        path.to_owned()
    } else {
        path.rsplit('/').next().unwrap_or(path).to_owned()
    }
}
