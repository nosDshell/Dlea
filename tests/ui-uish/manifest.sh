#!/usr/bin/env bash
# Declarations name overrides, not successful cases or native acceptance.
declare -a dlea_uish_modules=()
declare -A dlea_uish_assignments=()
dlea_uish_module() {
    local module=$1
    shift
    dlea_uish_modules+=("$module")
    dlea_uish_assignments["$module"]="$*"
}
dlea_uish_module tests/ui-uish/common-observers.sh scroll colour lifted settings
dlea_uish_module tests/ui-uish/launch.sh select renderer
dlea_uish_module tests/ui-uish/state.sh xwsettings xwwatch makedefault
dlea_uish_module tests/ui-uish/sizes.sh dirsortstale nosweep hiddenlast hanginspect
dlea_uish_module tests/ui-uish/inline-rename.sh clickedge renamefirst renamelife renamefar
dlea_uish_module tests/ui-uish/modal-routing.sh collide menu clickthrough wheelunder
dlea_uish_module tests/ui-uish/places.sh editplace placemenu rename recent
dlea_uish_module tests/ui-uish/network-form.sh network netmark networkauth
dlea_uish_module tests/ui-uish/network-service.sh networktimeout gvfs unmount hangshare hanglisting
dlea_uish_module tests/ui-uish/devices.sh unmounted optical phones eject poweroff
dlea_uish_module tests/ui-uish/providers.sh sharebrowser taildrop providers
dlea_uish_module tests/ui-uish/thumbnails.sh thumbs hashcache stale userthumbnailer
dlea_uish_module tests/ui-uish/quick-look.sh open preview lookopen relstart
dlea_uish_module tests/ui-uish/documents.sh pdffocus columns views formats
dlea_uish_module tests/ui-uish/media.sh mute previewviews
dlea_uish_module tests/ui-uish/preview-swap.sh watch previewswap
dlea_uish_module tests/ui-uish/tabs.sh tabs tabdrag middleclick opentab folderviews
dlea_uish_module tests/ui-uish/operations.sh dd ddclick operations transferlive xwundo
dlea_uish_module tests/ui-uish/open-with.sh openterminal makeexec openwithdesign
dlea_uish_module tests/ui-uish/drag.sh griddrop railpointer dualautohidereveal dualdivider
dlea_uish_module tests/ui-uish/menus.sh querypaste runscript background hidden reload focus columnsbackground rows
dlea_uish_module tests/ui-uish/backend.sh terminal

dlea_uish_module tests/ui-uish/autofit.sh columnautofit
dlea_uish_module tests/ui-uish/input.sh shiftscroll overlays

dlea_uish_load() {
    [[ "$test_frontend" == gpui ]] || return 0
    python3 -B "$repo/tests/lib/dlea_uish_inventory.py" --repo "$repo" --check >/dev/null || return 1
    local module name
    for module in "${dlea_uish_modules[@]}"; do
        if [[ ! -f "$repo/$module" ]]; then
            [[ "$module" == tests/ui-uish/common-observers.sh ]] && continue
            printf 'missing GPUI override module: %s\n' "$module" >&2
            return 1
        fi
        . "$repo/$module" || return 1
        for name in ${dlea_uish_assignments[$module]}; do
            declare -F "case_$name" >/dev/null || return 1
        done
    done
}
