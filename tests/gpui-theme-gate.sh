#!/usr/bin/env bash
# Stock contrast and real GPUI startup/theme subscription, always in a marked child HOME.
set -euo pipefail
# Cargo starts the test binary through --runner; env -i leaves it only PATH, LD_LIBRARY_PATH, the sandbox dirs and the gate's DLEA_ variables.
if [[ ${1-} == --runner ]]; then
    shift
    root=${DLEA_THEME_GATE_ROOT:?runner without the gate root}
    pass=(PATH="$PATH" HOME="$root/home" XDG_CONFIG_HOME="$root/config" XDG_CACHE_HOME="$root/cache" XDG_STATE_HOME="$root/state"
        XDG_DATA_HOME="$root/home/.local/share" XDG_RUNTIME_DIR="$root/runtime" DLEA_THEME_GATE_ROOT="$root"
        DLEA_BIN="$DLEA_BIN" DLEA_THEME_STOCK_DIR="$DLEA_THEME_STOCK_DIR" DLEA_FIXTURE_ROOT="$DLEA_FIXTURE_ROOT")
    [[ ! -v LD_LIBRARY_PATH ]] || pass+=(LD_LIBRARY_PATH="$LD_LIBRARY_PATH")
    exec env -i "${pass[@]}" "$@"
fi
repo=$(cd "$(dirname "$0")/.." && pwd)
cd "$repo"
# This suite owns only /tmp fixtures; build products stay in the caller's target directory.
fixture_parent=$(mktemp -d /tmp/dlea-gpui-theme-gate.XXXXXXXX)
# Until the marker lands both dirs are empty, so rmdir is the whole cleanup for an early exit.
trap 'rmdir -- ${test_root:+"$test_root"} "$fixture_parent"' EXIT
export DLEA_FIXTURE_ROOT="$fixture_parent"
. "$repo/tools/dlea-sandbox-guard"
test_root=$(mktemp -d "$fixture_parent/run.XXXXXXXX")
sandbox_require "$test_root"
: > "$test_root/$SANDBOX_MARKER"
trap 'sandbox_remove "$test_root"; rmdir "$fixture_parent"' EXIT
for dir in home config cache state runtime; do mkdir -p "$test_root/$dir"; done
chmod 700 "$test_root/runtime"
: > "$test_root/home/$SANDBOX_MARKER"
: > "$test_root/runtime/$SANDBOX_MARKER"
stock=${DLEA_THEME_STOCK_DIR:-/usr/share/omarchy/themes}
[[ -d $stock ]] || { echo "FAIL gpui-theme-gate: missing stock inventory $stock" >&2; exit 1; }
# Whatever stock themes the pinned Omarchy installs, plus the Cool Dawn and Sunset fixtures.
fixture_palettes=2
count=0
for palette in "$stock"/*/; do
    if [[ -f ${palette}colors.toml ]]; then count=$((count + 1)); fi
done
(( count > 0 )) || { echo "FAIL gpui-theme-gate: no stock palettes in $stock" >&2; exit 1; }
expected=$((count + fixture_palettes))
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$repo/target}
# Built with the caller's profile while HOME still serves rustup, so a CI lane's prep-built dlea is reused, not rebuilt.
cargo build --offline --locked -p dlea --bin dlea
# Line tables only, as every other dlea-gui lib-test suite builds, keep this build inside the CI lane memory bound.
export CARGO_PROFILE_DEV_DEBUG=line-tables-only CARGO_PROFILE_TEST_DEBUG=line-tables-only
cargo test --offline --locked -p dlea-gui --lib theme_gate --no-run
# Cargo keeps the caller's environment, as the --no-run build above did, so nothing it fingerprinted changes.
[[ $repo != *"'"* ]] || { echo "FAIL gpui-theme-gate: the runner path cannot hold a quote: $repo" >&2; exit 1; }
export DLEA_BIN="$CARGO_TARGET_DIR/debug/dlea" DLEA_THEME_GATE_ROOT="$test_root" DLEA_THEME_STOCK_DIR="$stock"
cargo test --offline --locked -p dlea-gui --lib theme_gate --config "target.'cfg(all())'.runner = ['/bin/bash', '$repo/tests/gpui-theme-gate.sh', '--runner']" \
    -- --ignored --nocapture --test-threads=1
printf 'GPUI_THEME_GATE_CASES stock=%s fixtures=%s palettes=%s\n' "$count" "$fixture_palettes" "$expected"
printf 'GPUI_THEME_GATE PASS stock=%s contrast and live atomic replacement\n' "$count"
