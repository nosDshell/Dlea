#!/usr/bin/env bash
. "$(dirname "$0")/headless-commons.sh"
# A Markdown document with figures starts the figure helper warm when it rests; Source view, a plain document and a moving cursor start nothing.
set -u
. "$(dirname "$0")/../tools/dlea-sandbox-guard"
cd "$(dirname "$0")/.." || exit 1
command -v qs >/dev/null || { echo "figure-warm.sh: qs is not installed"; exit 1; }
resolve_qjs() {
    if [ -n "${DLEA_QJS:-}" ] && [ "${DLEA_QJS#/}" != "${DLEA_QJS}" ] && [ -x "${DLEA_QJS}" ]; then
        printf '%s\n' "${DLEA_QJS}"
    elif [ -x /usr/bin/qjs ]; then
        printf '%s\n' /usr/bin/qjs
    elif [ -x "$PWD/.superpowers/tools/qjs" ]; then
        printf '%s\n' "$PWD/.superpowers/tools/qjs"
    else
        return 1
    fi
}
qjs=$(resolve_qjs) || { echo "figure-warm.sh: no qjs (DLEA_QJS, /usr/bin/qjs)"; exit 1; }
dleabin="$PWD/target/debug/dlea"
[ -x "$dleabin" ] || { echo "figure-warm.sh: no debug binary at $dleabin, run cargo build first"; exit 1; }
export DLEA_UI="$PWD/ui"
# The warm start is what is under test, not the cache.
export DLEA_FIGURE_CACHE=off

test_root="$FIXTURE_ROOT/dlea-figure-warm-$$"
sandbox_make "$test_root"
trap 'sandbox_remove "$test_root"' EXIT
mkdir -p "$test_root/config" "$test_root/home" "$test_root/state" "$test_root/cache" "$test_root/runtime" || exit 1
chmod 700 "$test_root/runtime" || exit 1
ln -s "$PWD/ui" "$test_root/config/dlea" || exit 1
headless_commons "$test_root" || exit 1
ln -s "$(readlink -f ui/boot/Ui)" "$test_root/config/Ui" || exit 1
cp tests/figure-warm.qml "$test_root/config/shell.qml" || exit 1

# The jail the helper runs in needs user namespaces; a box without them cannot show a warm helper, and says so.
probe=$(printf '%s\n' '{"id":1,"kind":"math","source":"x","display":false,"theme":{"bg":"#101315","fg":"#c0caf5"}}' | DLEA_QJS="$qjs" "$dleabin" --figure-helper 2>&1)
if ! printf '%s\n' "$probe" | grep -q '"svg"'; then
    if printf '%s\n' "$probe" | grep -q '^bwrap: No permissions to create new namespace'; then
        echo "figure-warm.sh: SKIP no user namespaces in this container, so no jailed helper to warm"
        exit 0
    fi
    printf 'figure-warm.sh: FAIL the helper did not answer: %s\n' "$probe"
    exit 1
fi

printf '# Figures\n\n```mermaid\nflowchart TD\n    A --> B\n```\n\n```math\n\\frac{a}{b}\n```\n' > "$test_root/figures.md"
printf '# Plain\n\nJust prose and `code`.\n\n```js\nvar a = 1;\n```\n' > "$test_root/plain.md"

output=$( ( env -u DISPLAY -u WAYLAND_DISPLAY -u HYPRLAND_INSTANCE_SIGNATURE \
    HOME="$test_root/home" XDG_STATE_HOME="$test_root/state" XDG_CACHE_HOME="$test_root/cache" \
    XDG_RUNTIME_DIR="$test_root/runtime" DLEA_BIN="$dleabin" DLEA_QJS="$qjs" DLEA_UI="$DLEA_UI" \
    DLEA_FIGURE_WARM_FIGURES="$test_root/figures.md" DLEA_FIGURE_WARM_PLAIN="$test_root/plain.md" \
    QT_QPA_PLATFORM=offscreen QT_QPA_PLATFORMTHEME=generic QT_QUICK_BACKEND=software QT_QPA_UPDATE_IDLE_TIME=1 QT_FORCE_STDERR_LOGGING=1 \
    timeout 120 qs -p "$test_root/config" 2>&1 ) 2>/dev/null )
printf '%s\n' "$output" | grep -aoE 'FIGURE_WARM .*'
# Sample input: FIGURE_WARM 10 checks, 0 failed.
expected=10
if ! printf '%s\n' "$output" | grep -qE "FIGURE_WARM $expected checks, 0 failed$"; then
    printf 'figure-warm.sh: FAIL expected %s checks, 0 failed\n' "$expected"
    printf '%s\n' "$output" | grep -aE 'ERROR|TypeError|ReferenceError|dlea:' | head -10
    exit 1
fi
platform_warning='This plugin does not support setting window masks'
warnings=$(printf '%s\n' "$output" | grep -aE 'TypeError|ReferenceError|WARN|invalid nullptr parameter' | grep -vF "$platform_warning")
if [ -n "$warnings" ]; then
    printf 'figure-warm.sh: FAIL the harness logged a warning\n%s\n' "$warnings" | head -10
    exit 1
fi
echo "figure-warm: $expected check(s), 0 failed"
