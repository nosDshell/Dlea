#!/bin/bash
# Shell entry points share the owned-window implementation with Python harnesses.
dlea_test_helper=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/dlea_transport_cli.py

dlea_test_launch() {
    local path=$1 log=$2
    shift 2
    export DLEA_TEST_OWNER_PID=$$
    DLEA_TEST_WINDOW=$(python3 -B "$dlea_test_helper" start "$path" --log "$log" "$@") || return $?
    export DLEA_TEST_WINDOW
    DLEA_TEST_PID=$(python3 -B "$dlea_test_helper" pid "$DLEA_TEST_WINDOW") || return $?
}

dlea_test_call() { python3 -B "$dlea_test_helper" call "$DLEA_TEST_WINDOW" "$@"; }
dlea_test_quit() { python3 -B "$dlea_test_helper" quit "$DLEA_TEST_WINDOW"; }
dlea_test_stop() { python3 -B "$dlea_test_helper" stop "$DLEA_TEST_WINDOW"; }
dlea_test_stop_all() { python3 -B "$dlea_test_helper" stop-all "$DLEA_TEST_WINDOWS_ROOT"; }

dlea_test_exec() {
    export DLEA_TEST_OWNER_PID=$$
    python3 -B "$dlea_test_helper" exec -- "$@"
}

dlea_test_running() {
    local pattern=$1 found=0 line
    while IFS= read -r line; do
        [ "$line" = "$pattern" ] || continue
        found=1
    done < <(ps -eo args=)
    if [ "$found" = 1 ]; then
        ps -eo pid=,args= | while read -r pid rest; do
            [ "$rest" = "$pattern" ] && printf '%s\n' "$pid"
        done
        return 0
    fi
    return 1
}
