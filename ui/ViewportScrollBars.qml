import QtQuick
import "." as Dlea
import "js/Scroll.js" as Scroll

// Both axes of a pannable document, sharing the trailing corner rather than painting through it.
Item {
    id: root
    required property var flickable
    anchors.fill: parent
    z: Scroll.BAR_Z

    Dlea.ViewportScrollBar {
        parent: root
        anchors { top: parent.top; right: parent.right }
        flickable: root.flickable
        endInset: Theme.spacing.rowPaddingX
    }
    Dlea.ViewportScrollBar {
        parent: root
        anchors { left: parent.left; bottom: parent.bottom }
        flickable: root.flickable
        orientation: Qt.Horizontal
        endInset: Theme.spacing.rowPaddingX
    }
}
