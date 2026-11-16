//@ pragma ShellId dlea-sortnew-test
import QtQuick
import Quickshell
import "dlea" as Dlea
import "dlea/js/Sort.js" as Sort

// The same fixture goes through real views, peeks, picker navigation and a persisted Name choice.
ShellRoot {
    id: root
    readonly property string home: Quickshell.env("HOME")
    readonly property string downloads: root.home + "/Inbox"
    readonly property var newest: ["folder", "z-new.txt", "a-old.txt", ".hidden.txt"]
    readonly property var named: ["folder", "a-old.txt", "z-new.txt", ".hidden.txt"]
    property var win: null
    property var peekNames: null
    property var steps: []
    property int at: 0
    property int checks: 0
    property int failures: 0
    property bool armed: false
    property double stamp: Date.now()

    function check(label, got, want) {
        checks++
        var ok = JSON.stringify(got) === JSON.stringify(want)
        if (!ok) failures++
        console.log("SORTNEW " + (ok ? "PASS " : "FAIL ") + label + " got=" + JSON.stringify(got) + " expected=" + JSON.stringify(want))
    }
    function names(rows) { return rows.map(function (row) { return row.n }) }
    function paneRows(path) { return pane.path === path && !pane.listInFlight && pane.rows.length === pane.total ? names(pane.rows) : null }
    function pickerRows(path) { return win.path === path && win.listingState === "ready" && win.rows.length === win.total ? names(win.rows) : null }
    function listing(label, path, want, view) {
        return { label: label, view: view || "list", start: function () {
            Dlea.ViewState.changeKey("view", view || "list")
            pane.openWithoutHistory(path)
        }, read: function () { return paneRows(path) }, want: want }
    }
    function pickerListing(label, path, want) {
        return { label: label, start: function () { win.openWithoutHistory(path) },
            read: function () { return pickerRows(path) }, want: want }
    }
    function finish() {
        console.log("SORTNEW DONE " + checks + " checks, " + failures + " failed")
        Qt.exit(failures ? 1 : 0)
    }

    Dlea.Backend { id: be }
    Connections {
        target: be
        function onPeeked(path, hidden, total, rows) { if (path === root.downloads) root.peekNames = root.names(rows) }
    }
    FloatingWindow {
        implicitWidth: 1200
        implicitHeight: 640
        color: Dlea.Theme.color.background
        Dlea.Pane { id: pane; anchors.fill: parent; backend: be; showHidden: true }
    }

    Component.onCompleted: {
        var comp = Qt.createComponent("dlea/PickerWindow.qml")
        if (comp.status !== Component.Ready) { check("picker compiles", comp.errorString(), ""); finish(); return }
        root.win = comp.createObject(root).pickerWin
        root.win.showHidden = true
        Dlea.ViewState.changeKey("hidden", true)
        Dlea.ViewState.changeKey("hiddenLast", true)
        Dlea.ViewState.changeKey("foldersFirst", true)
        if (Quickshell.env("SORTNEW_PHASE") === "restart") {
            root.steps = [listing("Name ascending survives a new process", downloads, named)]
            return
        }
        var folders = [downloads, home + "/Photo Library", home + "/Clips"]
        var views = ["list", "grid", "columns"]
        for (var v = 0; v < views.length; v++) {
            for (var f = 0; f < folders.length; f++) root.steps.push(listing(views[v] + " " + folders[f] + " newest first", folders[f], newest, views[v]))
        }
        root.steps.push({ label: "the rail reads the same custom XDG config", start: function () {},
            read: function () { return Dlea.RailPlaces.homeEntries.map(function (entry) { return entry.path }) },
            want: [home, downloads, home + "/Photo Library", home + "/Clips", home + "/Documents"] })
        root.steps.push({ label: "header marks Modified newest first", start: function () {},
            read: function () { return [pane.header.sortBy, pane.header.sortDesc] }, want: ["mtime", true] })
        root.steps.push({ label: "header draws the descending Modified arrow", start: function () {},
            read: function () { return pane.header.title("Modified", "mtime") }, want: "Modified \u25be" })
        root.steps.push(listing("Documents keeps name order", home + "/Documents", named))
        root.steps.push({ label: "a Downloads child column peeks newest first", start: function () {
            Dlea.ViewState.changeKey("view", "columns")
            pane.openWithoutHistory(home)
        }, read: function () {
            if (pane.path !== home || pane.listInFlight || !pane.columnsArea) return null
            for (var i = 0; i < pane.total; i++) {
                var row = pane.rowFor(i)
                if (row && row.n === "Inbox") { if (pane.cursorIndex !== i) pane.setCursor(i); break }
            }
            var rows = pane.columnsArea.rowsFor(downloads)
            return rows.length > 0 ? names(rows) : null
        }, want: newest })
        root.steps.push(pickerListing("picker Downloads newest first", downloads, newest))
        root.steps.push({ label: "picker marks Modified newest first", start: function () {},
            read: function () { return [win.sortBy, win.sortDesc] }, want: ["mtime", true] })
        root.steps.push(pickerListing("picker navigation restores Documents name order", home + "/Documents", named))
        root.steps.push({ label: "external global sort does not relabel the picker's held rows", start: function () {
            Dlea.ViewState.changeLeaf("sort", { key: "size", reverse: true })
        }, read: function () {
            if (Object.keys(Dlea.ViewState.unsaved).length > 0) return null
            return [win.sortBy, win.sortDesc]
        }, want: ["name", false] })
        root.steps.push(pickerListing("picker navigation restores Downloads newest first", downloads, newest))
        root.steps.push({ label: "remember off uses global Name ascending", start: function () {
            Dlea.ViewState.changeKey("rememberSort", false)
            Dlea.ViewState.changeLeaf("sort", { key: "name", reverse: false })
            pane.openWithoutHistory(downloads)
        }, read: function () { return paneRows(downloads) }, want: named })
        root.steps.push({ label: "remember off marks global Name ascending", start: function () {},
            read: function () { return [be.sortBy, be.sortDesc] }, want: ["name", false] })
        root.steps.push(pickerListing("remember off picker uses global Name ascending", downloads, named))
        root.steps.push({ label: "remember off column peek uses global Name ascending", start: function () {
            root.peekNames = null
            be.peek(downloads, 50, true, undefined, [downloads])
        }, read: function () { return root.peekNames }, want: named })
        root.steps.push({ label: "default browsing stores no entries", start: function () {},
            read: function () { return Object.keys(Dlea.ViewState.state.folderSorts || {}) }, want: [] })
        root.steps.push({ label: "remember on restores the default", start: function () {
            Dlea.ViewState.changeKey("rememberSort", true)
            Dlea.ViewState.changeLeaf("sort", { key: "name", reverse: false })
            pane.openWithoutHistory(downloads)
        }, read: function () { return paneRows(downloads) }, want: newest })
        root.steps.push({ label: "explicit Name ascending sorts and persists", start: function () { Sort.resort(pane, "name", false) },
            read: function () {
                if (Object.keys(Dlea.ViewState.unsaved).length > 0) return null
                return paneRows(downloads)
            }, want: named })
        root.steps.push(listing("explicit Name wins on reopen", downloads, named))
    }

    Timer {
        interval: 20
        repeat: true
        running: true
        onTriggered: {
            if (Date.now() - root.stamp > 15000) {
                console.log("SORTNEW STATE armed=" + root.armed + " pane=" + pane.path + " total=" + pane.total + " rows=" + pane.rows.length
                    + " waiting=" + pane.listInFlight + " picker=" + (root.win ? root.win.listingState : "absent"))
                root.check("step " + root.at + " answers", "timeout", "answer"); root.finish(); return
            }
            if (!root.win || root.win.listingState !== "ready") return
            if (root.at >= root.steps.length) { root.finish(); return }
            var step = root.steps[root.at]
            if (!root.armed) { root.armed = true; root.stamp = Date.now(); step.start(); return }
            var got = step.read()
            if (got === null || got === undefined) return
            root.check(step.label, got, step.want)
            if (step.view) root.check(step.label + " view is active", pane.viewMode, step.view)
            root.armed = false
            root.at++
            root.stamp = Date.now()
        }
    }
}
