#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# Both frontends run before the verdict, so one run reports every broken row.
status=0
python3 tests/compat/cli_contract.py "${DLEA_BIN:-$PWD/target/debug/dlea}" "$@" || status=1
python3 tests/compat/cli_contract.py "${DLEA_BIN:-$PWD/target/debug/dlea}" --gpui "$@" || status=1
exit "$status"
