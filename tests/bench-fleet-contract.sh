#!/bin/bash
# Validates offline fleet inventories and deliberate identity/refusal controls without launching entrants.
set -euo pipefail
cd "$(cd "$(dirname "$0")/.." && pwd -P)"
mkdir -p "$PWD/.superpowers/m9-bench-05"
root=$(mktemp -d "$PWD/.superpowers/m9-bench-05/contract.XXXXXXXX")
: > "$root/.dlea-test-sandbox"
trap 'case $root in "$PWD"/.superpowers/m9-bench-05/contract.?*) [ -n "$root" ] && [ -f "$root/.dlea-test-sandbox" ] && rm -rf -- "$root" ;; esac' EXIT
python3 -B tests/bench-fleet-contract.py "$root"
