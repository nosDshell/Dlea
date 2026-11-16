import QtQuick
import "." as Dlea

// The wheel and the two knobs of a zoomed picture; the Loader that builds this sits on the Flickable they drive.
Item {
    id: root

    readonly property Item viewport: root.parent ? root.parent.parent : null
    anchors.fill: parent

    Dlea.FastScrollHandler { flickable: root.viewport }
    Dlea.ViewportScrollBars { flickable: root.viewport }
}
