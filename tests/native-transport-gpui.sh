#!/bin/bash
# Run prebuilt product binaries through owned IPC on a real display.
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
cd "$repo"
export DLEA_TEST_FRONTEND=gpui
. "$repo/tests/lib/dlea-transport-ci.sh"
dlea_transport_ci "$0" "$@"
mkdir -p "$repo/.superpowers/g010t"
if [[ -z ${DISPLAY:-}${WAYLAND_DISPLAY:-} && -z ${DLEA_TEST_PRIVATE_X11:-} ]]; then
    for tool in Xvfb bwrap; do
        command -v "$tool" >/dev/null || { printf 'TRANSPORT_NATIVE BLOCKED: %s is required without a graphical session\n' "$tool" >&2; exit 1; }
    done
    root=$(mktemp -d "$repo/.superpowers/g010t/x11.XXXXXX")
    : > "$root/.dlea-test-sandbox"
    # /tmp itself stays unmarked so the escape controls can still create an untrusted directory.
    mkdir -m 1777 "$root/tmp"
    exec bwrap --bind / / --dev /dev --bind "$root/tmp" /tmp --setenv DLEA_TEST_PRIVATE_X11 1 -- bash "$0" "$@"
fi
for binary in dlea dlea-gui; do
    [[ -x ${CARGO_TARGET_DIR:-$repo/target}/debug/$binary ]] || {
        printf 'TRANSPORT_NATIVE BLOCKED: build %s through the build helper first\n' "$binary" >&2
        exit 1
    }
done
if [[ -z ${DISPLAY:-}${WAYLAND_DISPLAY:-} ]]; then
    command -v Xvfb >/dev/null || { echo 'TRANSPORT_NATIVE BLOCKED: Xvfb is required without a graphical session' >&2; exit 1; }
    root=$(mktemp -d "$repo/.superpowers/g010t/xvfb.XXXXXX")
    : > "$root/.dlea-test-sandbox"
    exec python3 -B "$repo/tests/lib/dlea_transport_xvfb.py" "$root" bash "$repo/tests/native-transport.sh" "$@"
fi
bash "$repo/tests/native-transport.sh" "$@"
