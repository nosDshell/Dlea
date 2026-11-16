// Language-mark reference sheet; tests/langmarks3.sh builds it offscreen through the original symlink harness.
import Quickshell
import QtQuick
import "." as Dlea
import "js/LangMarks.js" as LangMarks
import "js/Density.js" as Density

ShellRoot {
    id: root
    FloatingWindow {
        id: win
        implicitWidth: 640
        implicitHeight: 560
        visible: true
        title: "langmarks-spec"
        color: Dlea.Theme.color.background

        // The same three inks ui/Row.qml binds its mark to: dim at rest, foreground lifted.
        readonly property color restInk: Dlea.Theme.color.metadataFrame
        readonly property var files: ["main.rs", "app.py", "index.js", "README.md", "Dockerfile"]

        Column {
            anchors.fill: parent
            anchors.margins: 24
            spacing: 10

            // A folder row keeps its kind glyph; every other row below replaces it with a logo.
            Row {
                spacing: 12
                Text { width: 140; text: "folder"; color: Dlea.Theme.color.muted; font.pixelSize: Dlea.Theme.font.caption; textFormat: Text.PlainText }
                Dlea.Glyph { width: Dlea.Theme.markSize; height: Dlea.Theme.markSize; name: "folder"; color: win.restInk }
                Dlea.Glyph { width: Dlea.Theme.markSize; height: Dlea.Theme.markSize; name: "folder"; color: Dlea.Theme.color.foreground }
                Dlea.Glyph { width: Dlea.Theme.markSize; height: Dlea.Theme.markSize; name: "folder"; color: Dlea.Theme.color.foreground }
            }

            // An unmapped file keeps its kind glyph too.
            Row {
                spacing: 12
                Text { width: 140; text: "notes.txt"; color: Dlea.Theme.color.muted; font.pixelSize: Dlea.Theme.font.caption; textFormat: Text.PlainText }
                Dlea.Glyph { width: Dlea.Theme.markSize; height: Dlea.Theme.markSize; name: "file-text"; color: win.restInk }
                Dlea.Glyph { width: Dlea.Theme.markSize; height: Dlea.Theme.markSize; name: "file-text"; color: Dlea.Theme.color.foreground }
                Dlea.Glyph { width: Dlea.Theme.markSize; height: Dlea.Theme.markSize; name: "file-text"; color: Dlea.Theme.color.foreground }
            }

            Repeater {
                model: win.files
                Row {
                    required property string modelData
                    spacing: 12
                    Text { width: 140; text: parent.modelData; color: Dlea.Theme.color.muted; font.pixelSize: Dlea.Theme.font.caption; textFormat: Text.PlainText }
                    Dlea.LangMark {
                        width: Dlea.Theme.markSize; height: Dlea.Theme.markSize
                        maxSize: Dlea.Theme.markSize; slug: LangMarks.slugForFile(parent.modelData)
                        color: win.restInk; strokeWidth: Dlea.Theme.strokeWidth
                    }
                    Dlea.LangMark {
                        width: Dlea.Theme.markSize; height: Dlea.Theme.markSize
                        maxSize: Dlea.Theme.markSize; slug: LangMarks.slugForFile(parent.modelData)
                        color: Dlea.Theme.color.foreground; strokeWidth: Dlea.Theme.strokeWidth
                    }
                    Dlea.LangMark {
                        width: Dlea.Theme.markSize; height: Dlea.Theme.markSize
                        maxSize: Dlea.Theme.markSize; slug: LangMarks.slugForFile(parent.modelData)
                        color: Dlea.Theme.color.foreground; strokeWidth: Dlea.Theme.strokeWidth
                    }
                }
            }

            // One grid cell at the Medium tile's grid mark size, muted as the tile's glyph.
            Row {
                spacing: 12
                Text { width: 140; text: "grid main.rs"; color: Dlea.Theme.color.muted; font.pixelSize: Dlea.Theme.font.caption; textFormat: Text.PlainText }
                Dlea.LangMark {
                    width: 64; height: 64
                    maxSize: Density.glyphCap(64); slug: "rust"
                    color: Dlea.Theme.color.mutedFrame; strokeWidth: Dlea.Theme.strokeWidth
                }
            }
        }
    }
    Timer {
        interval: 800
        running: Quickshell.env("LANGMARK_SPEC_TEST") === "1"
        onTriggered: {
            var count = 0, stack = [win.contentItem]
            while (stack.length > 0) {
                var item = stack.pop()
                if (String(item).indexOf("LangMark_") === 0) count += 1
                var children = item.children || []
                for (var i = 0; i < children.length; i++) stack.push(children[i])
            }
            console.log("LANGMARK_SPEC " + (count === 16 ? "PASS" : "FAIL") + " marks=" + count)
            Quickshell.execDetached(["kill", String(Quickshell.processId)])
        }
    }

}
