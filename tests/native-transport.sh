#!/bin/bash
# The same owned transport drives the QML browser and the native browser.
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
cd "$repo"
. "$repo/tests/lib/dlea-transport-ci.sh"
dlea_transport_ci "$0" "$@"
if [[ ${1:-} != controls && ${DLEA_TEST_FRONTEND:-gpui} == gpui && -z ${DISPLAY:-}${WAYLAND_DISPLAY:-} ]]; then
    exec bash "$repo/tests/native-transport-gpui.sh" "$@"
fi
mkdir -p "$repo/.superpowers/g010t"
python3 -B "$repo/tests/lib/dlea_transport_controls.py"
if [[ ${1:-} == controls ]]; then exit 0; fi
root=$(mktemp -d "$repo/.superpowers/g010t/native.XXXXXX")
: > "$root/.dlea-test-sandbox"
export DLEA_TEST_WINDOWS_ROOT="$root"
if [[ ${1:-} == material ]]; then
    [[ ${DLEA_TEST_FRONTEND:-gpui} == gpui ]] || { echo 'material product proof requires GPUI' >&2; exit 2; }
    exec python3 -B "$repo/tests/lib/dlea_transport_material.py" "$root"
fi
if [[ ${DLEA_TEST_FRONTEND:-gpui} == qml ]]; then
    . tools/dlea-sandbox-guard
    . tests/headless-commons.sh
    FIXTURE_ROOT="$root"
    cp -a ui "$root/ui"
    mkdir "$root/work"
    headless_commons "$root/work" "$root/work/commons" || exit 1
    python3 -B - "$root" <<'PY'
from pathlib import Path
import sys
root = Path(sys.argv[1])
assert root.is_absolute() and (root / '.dlea-test-sandbox').is_file()
ui = root / 'ui'
for link in (ui / 'Commons', ui / 'boot' / 'Commons'):
    if link.is_symlink():
        assert str(link) and link.is_absolute() and link.is_relative_to(root)
        link.unlink()
        link.symlink_to(root / 'work/commons', target_is_directory=True)
PY
    export DLEA_TEST_UI="$root/ui" QT_QPA_PLATFORM=offscreen QT_QPA_PLATFORMTHEME=generic QSG_RHI_BACKEND=opengl
    export DISPLAY=:0
    unset WAYLAND_DISPLAY HYPRLAND_INSTANCE_SIGNATURE
fi
python3 -B "$repo/tests/lib/dlea_transport_native.py" "$root"
