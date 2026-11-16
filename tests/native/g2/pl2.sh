#!/bin/bash
# Controller-only network acceptance; the shared harness owns the isolated window and fixtures.
set -euo pipefail
pl2_repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd -P)
: "${DLEA_BIN:?set the exact staged candidate or frozen oracle binary}"
source "$pl2_repo/tools/dlea-ipc"
dlea_ipc_init "$DLEA_BIN" "${DLEA_FRONTEND:-gpui}"
export DLEA_FRONTEND=${DLEA_FRONTEND:-gpui}
if [ "$#" -eq 0 ]; then
    set -- network netmark networkauth networktimeout gvfs_cleanup gvfs sharebrowser hangshare hanglisting hanginspect
fi
exec /bin/bash "$pl2_repo/tests/ui.sh" "$@"
