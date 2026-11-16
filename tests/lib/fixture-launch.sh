#!/bin/bash
# Sourced over the shared transport by cases whose launch target is a double: spawns the --binary given with its --ui, wrap prefix kept.
dlea_test_launch() {
    local start_path="$1" dlea_log="$2" ui="${dlea_ui-}" bin="${dlea_bin-}" backend="" arg
    shift 2
    local -a wrap=()
    while [ $# -gt 0 ]; do
        arg=$1
        case "$arg" in
            --ui|--binary|--backend)
                [ $# -ge 2 ] || fail "fixture launch got $arg with no value"
                case "$arg" in --ui) ui=$2 ;; --binary) bin=$2 ;; *) backend=$2 ;; esac
                shift ;;
            --ui=*) ui=${arg#--ui=} ;;
            --binary=*) bin=${arg#--binary=} ;;
            --backend=*) backend=${arg#--backend=} ;;
            --wrap=*) wrap+=("${arg#--wrap=}") ;;
            *) fail "fixture launch takes only --ui, --binary, --backend and --wrap, got $arg" ;;
        esac
        shift
    done
    [ -n "$bin" ] || fail "fixture launch has no binary"
    DLEA_UI="$ui" DLEA_BIN="${backend:-$bin}" ${wrap[@]+"${wrap[@]}"} \
        setsid nohup "$bin" --gui "$start_path" >"$dlea_log" 2>&1 </dev/null &
    DLEA_TEST_PID=$!
}
