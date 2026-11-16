#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
python3 tests/compat/legacy_upgrade.py "${DLEA_BIN:-$PWD/target/debug/dlea}" --helpers
