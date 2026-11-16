import QtQuick
Item {
    IpcHandler {
        target: "dleapicker"
        function ready(): bool { return true }
    }
}
