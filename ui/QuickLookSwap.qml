import QtQuick
import "." as Dlea

// Quick Look wrapper, built on first open; it captures ui/Preview.qml's eager panes container.
Dlea.PreviewSwap {
    // Fills its Loader, or it measures zero and every hold answers at once.
    anchors.fill: parent
    property Item panesSource: null
    captureSource: panesSource
}
