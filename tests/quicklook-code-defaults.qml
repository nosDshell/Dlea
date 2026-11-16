//@ pragma ShellId dlea-quicklook-code-defaults-test
import QtQuick
import Quickshell
import "dlea" as Dlea

// Read the real singleton without a WindowBody or navigation writer materializing missing keys.
ShellRoot {
    id: root
    readonly property string mode: Quickshell.env("QLCODE_DEFAULTS")
    property int failures: 0
    function check(label, actual, expected) {
        var ok = JSON.stringify(actual) === JSON.stringify(expected)
        if (!ok) root.failures++
        console.log("QLDEFAULT " + (ok ? "PASS" : "FAIL") + " mode=" + mode + " " + label
            + " got=" + JSON.stringify(actual) + " expected=" + JSON.stringify(expected))
    }
    Component.onCompleted: {
        var state = Dlea.ViewState.state, seeded = mode === "seeded"
        root.check("initial file read completed", Dlea.ViewState.initialReadComplete, true)
        if (!seeded) root.check("minimal file stays minimal", state, mode === "empty" ? ({}) : ({langMarks: false}))
        root.check("wrap key presence", Dlea.ViewState.preview.wrap !== undefined, seeded)
        root.check("missing-wrap fallback", Dlea.ViewState.previewWrap, false)
        root.check("language-icons state", Dlea.ViewState.langMarks, mode === "empty")
        console.log("QLDEFAULT DONE mode=" + mode + " failures=" + root.failures)
        quitter.start()
    }
    Timer { id: quitter; interval: 10; onTriggered: Qt.exit(root.failures === 0 ? 0 : 1) }
}
