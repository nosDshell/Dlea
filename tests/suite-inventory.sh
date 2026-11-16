#!/usr/bin/env bash
# Check suite accounting and each independent refusal against owned fixture copies.
suite_inventory_layout() {
    local repo=$1 input=${DLEA_CI_INPUT_DIR:-/in}
    local layout=${DLEA_CI_LAYOUT:-$input/suite-inventory.layout}
    local evidence=${DLEA_CI_EVIDENCE:-$input/suite-evidence.txt}
    if [[ ${DLEA_CI_LAYOUT+x} || ${DLEA_CI_EVIDENCE+x} || ${DLEA_CI_INPUT_DIR+x} || -e "$layout" || -f "$input/worker-files.sha256" ]]; then
        "$repo/tools/dlea-suite-inventory" --layout "$layout" --evidence "$evidence"
    fi
}

main() {
    set -euo pipefail
    cd "$(dirname "${BASH_SOURCE[0]}")/.."
    repo=$PWD
    suite_inventory_layout "$repo"
    mkdir -p "$repo/.superpowers/g010i"
    test_root=$(mktemp -d "$repo/.superpowers/g010i/inventory.XXXXXX")
    : > "$test_root/.suite-inventory-test-root"
    cleanup() {
        [[ -n "$test_root" && "$test_root" = /* && "$test_root" == "$repo/.superpowers/g010i/inventory."* && -f "$test_root/.suite-inventory-test-root" ]] && rm -rf -- "$test_root"
    }
    trap cleanup EXIT
    python3 -B tests/fixtures/suite-inventory/check.py "$repo" "$test_root"
}

if [[ ${BASH_SOURCE[0]} == "$0" ]]; then
    main "$@"
fi
