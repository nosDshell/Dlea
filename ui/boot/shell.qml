//@ pragma AppId com.nosDshell.dlea
//@ pragma ShellId dlea
//@ pragma NativeTextRendering
//@ pragma CacheDir $BASE/dlea

import Quickshell
import QtQuick

// The entry is the window and imports Quickshell and QtQuick only, because a Quickshell config is
// served through qs: URLs, which Qt's disk cache refuses; see AGENTS.md "The first window".
ShellRoot {
    FloatingWindow {
        id: dleaWindow
        title: "Dlea"
        implicitWidth: 900
        implicitHeight: 600
        // Never seen: the body covers it on the first frame. ui/Theme.qml's own fallback, for the record.
        color: "#101315"
        property bool rendererFallbackStarted: false
        // True when a glass launch cleared the colour before the window was shown, which tests/glass-boot.sh reads.
        property bool clearedEarly: false

        // Every *Centre reader on the IPC seam is this: an item's painted box, reduced to the point a test clicks.
        function centreOf(item) {
            if (!item)
                return ""
            var rect = dleaWindow.itemRect(item)
            return Math.round(rect.x + rect.width / 2) + " " + Math.round(rect.y + rect.height / 2)
        }
        // "x y width height" in window pixels, for a test that asserts a card stays inside the window.
        function rectOf(item) {
            if (!item)
                return ""
            var rect = dleaWindow.itemRect(item)
            var left = Math.round(rect.x), top = Math.round(rect.y)
            return left + " " + top + " " + (Math.round(rect.x + rect.width) - left) + " " + (Math.round(rect.y + rect.height) - top)
        }
        // centreOf's sibling, "x width centre": the edges round because a click needs a whole pixel, the centre keeps three decimals because the misalignment it reads is half of one.
        function boxOf(item) {
            if (!item)
                return ""
            var rect = dleaWindow.itemRect(item)
            return Math.round(rect.x) + " " + Math.round(rect.width) + " " + (rect.x + rect.width / 2).toFixed(3)
        }

        // This directory cannot reach ui/js/Format.js through qs:, so its one call is written out.
        function fileUrl(path) {
            return "file://" + encodeURI(path).replace(/#/g, "%23").replace(/\?/g, "%3F")
        }

        // Before the first frame, so the window maps holding its chrome; see AGENTS.md "The first window".
        function loadBody() {
            if (bodyLoader.status !== Loader.Null)
                return
            bodyLoader.setSource(dleaWindow.fileUrl(Quickshell.shellDir + "/../WindowBody.qml"), { host: dleaWindow })
            // Only before the window exists does Quickshell still take a clear surface; see ui/Glass.qml.
            if (bodyLoader.item && bodyLoader.item.clearSurface) {
                dleaWindow.clearedEarly = !(bodyLoader.Window.window && bodyLoader.Window.window.visible)
                if (dleaWindow.clearedEarly) dleaWindow.color = "transparent"
            }
        }

        // Quickshell 0.3.1 has no exit API, and it is signalled from here because a window closed
        // before the body loads still has to take the process with it.
        Connections { target: Quickshell; function onLastWindowClosed() { dleaWindow.quit() } }

        // Nothing has been told to drain before the body exists, so an early exit kills directly.
        function quit() {
            if (bodyLoader.item)
                bodyLoader.item.quitBackends()
            else
                Quickshell.execDetached(["kill", String(Quickshell.processId)])
        }

        // sceneGraphError arrives on the first frame, and the entry always exists, so the retry is here.
        function handleSceneGraphError(error, message) {
            var backendName = Quickshell.env("QSG_RHI_BACKEND")
            console.warn("graphics backend " + backendName + " failed (" + error + "): " + message)
            var retry = retryCommand(backendName)
            if (retry && !rendererFallbackStarted) {
                rendererFallbackStarted = true
                Quickshell.execDetached(retry)
            }
            quit()
        }

        // Loaded only once the error has arrived, which keeps ui/js/Renderer.js off the startup path.
        function retryCommand(backendName) {
            var helper = Qt.createComponent(dleaWindow.fileUrl(Quickshell.shellDir + "/../RendererRetry.qml"))
            if (helper.status !== Component.Ready) {
                console.warn("dlea: the renderer retry helper did not load, so there is no fallback: " + helper.errorString())
                return null
            }
            var object = helper.createObject(dleaWindow)
            var retry = object.fallbackCommand(backendName)
            object.destroy()
            return retry
        }

        Loader {
            id: bodyLoader
            anchors.fill: parent
            // A Loader is a focus scope, so without this every key press lands nowhere.
            focus: true
            // An empty window forever is what this catches; the engine prints the reason above it.
            onStatusChanged: {
                if (status === Loader.Error) {
                    console.warn("dlea: the window body did not load, so there is nothing to show")
                    dleaWindow.quit()
                }
            }
        }

        // Null while this file loads and the QQuickWindow once it exists, which is before the scene graph starts.
        Connections {
            target: bodyLoader.Window.window
            function onSceneGraphError(error, message) { dleaWindow.handleSceneGraphError(error, message) }
        }

        Component.onCompleted: dleaWindow.loadBody()
    }
}
