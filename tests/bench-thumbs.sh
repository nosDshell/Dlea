#!/usr/bin/env bash
# Thumbnail benchmark contracts, without an entrant, operator cache, display, or package mutation.
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
root=$(mktemp -d /tmp/dlea-b7-contracts.XXXXXXXX)
touch -- "$root/.dlea-test-sandbox"
export DLEA_B7_TEST_ROOT="$root"
trap 'python3 -B tests/bench-thumbs.py --cleanup-root "$root"' EXIT
printf 'bench-thumbs: sandbox %s\n' "$root"
python3 -B tools/dlea-bench-thumbs --help >/dev/null
python3 -B tests/bench-thumbs.py
python3 -B tests/bench-thumbs-gpui.py
python3 -B tests/bench-thumbs-native.py
python3 -B tests/bench-thumbs-background.py
python3 -B tests/bench-thumbs-safety.py
