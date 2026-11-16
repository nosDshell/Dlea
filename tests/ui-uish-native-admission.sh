#!/bin/bash
# Retained-input controls exercise the mandatory native wallpaper entry point without a desktop.
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
cd "$repo"
. "$repo/tests/lib/dlea-transport-ci.sh"
dlea_transport_ci "$0" "$@"
python3 -B "$repo/tests/lib/dlea_native_envelope_controls.py"
