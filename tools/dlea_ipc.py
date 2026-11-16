"""Choose the IPC transport once from the binary and frontend a harness launches."""
import os
import json
import stat
import sys
from pathlib import Path


class Client:
    def __init__(self, binary="dlea", frontend=None):
        binary = str(binary)
        name = Path(binary).name
        self.kind = frontend or os.environ.get("DLEA_FRONTEND") or "gpui"
        if name in ("qs", "quickshell"):
            self.kind = "qml"
        elif name == "dlea-gui":
            self.kind = "gpui"
        if self.kind not in ("qml", "gpui"):
            raise ValueError(f"unknown frontend: {self.kind}")
        self.binary = os.environ.get("DLEA_BIN", "dlea") if name in ("qs", "quickshell", "dlea-gui") else binary
        self.helper = str(Path(__file__).resolve().with_name("dlea-ipc"))

    @staticmethod
    def start_owned(path, **options):
        sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tests/lib"))
        from dlea_transport import Window
        return Window.start(path, **options)

    def environment(self, environment=None):
        return dict(os.environ if environment is None else environment,
                    DLEA_IPC_KIND=self.kind, DLEA_IPC_BIN=self.binary, DLEA_IPC_HELPER=self.helper)

    def command(self, *arguments):
        return [self.helper, *map(str, arguments)]

    def export(self):
        os.environ.update(self.environment())


def instances():
    runtime = Path(os.environ["XDG_RUNTIME_DIR"])
    directory = runtime / "dlea-ipc"
    for path in (runtime, directory):
        info = path.lstat()
        if not stat.S_ISDIR(info.st_mode) or info.st_uid != os.geteuid() or info.st_mode & 0o077:
            raise PermissionError(f"IPC directory is not private: {path}")
    rows = []
    for socket in directory.glob("*.sock"):
        try:
            info = socket.lstat()
            # A file chooser binds picker-<pid>.sock; its config path names the chooser's boot file.
            stem = socket.stem.removeprefix("picker-")
            picker = stem != socket.stem
            pid = int(stem)
            if stem != str(pid) or pid < 1 or not stat.S_ISSOCK(info.st_mode):
                continue
            if info.st_uid != os.geteuid() or info.st_mode & 0o077:
                continue
            os.kill(pid, 0)
            environment = {}
            for field in Path(f"/proc/{pid}/environ").read_bytes().split(b"\0"):
                key, _, value = field.partition(b"=")
                if key in (b"DLEA_UI", b"DLEA_BIN"):
                    environment[os.fsdecode(key)] = os.fsdecode(value)
            binary = Path(environment.get("DLEA_BIN", "/usr/bin/dlea"))
            ui = environment.get("DLEA_UI")
            if not ui:
                ui = str(binary.parents[2] / "ui") if binary.parent.name in ("debug", "release") else "/usr/share/dlea/ui"
            rows.append({"id": str(pid), "pid": pid, "shell_id": "dleapicker" if picker else "dlea",
                         "config_path": str(Path(ui).resolve() / ("boot/picker.qml" if picker else "boot/shell.qml"))})
        except (OSError, ValueError):
            continue
    return sorted(rows, key=lambda row: row["pid"])


if __name__ == "__main__":
    try:
        rows = instances()
    except FileNotFoundError:
        rows = []
    if sys.argv[1:2] == ["pid"]:
        config = Path(sys.argv[2]).resolve()
        if config.suffix != ".qml":
            config = config / ("boot/shell.qml" if (config / "boot/shell.qml").is_file() else "shell.qml")
        matching = [row for row in rows if row["config_path"] == str(config)]
        if len(matching) != 1:
            print(f"dlea-ipc: {len(matching)} native windows run from {config}; use --pid", file=sys.stderr)
            sys.exit(255)
        print(matching[0]["pid"])
    elif "--json" in sys.argv:
        print(json.dumps(rows))
    else:
        for row in rows:
            print(f'Instance {row["id"]}:\n  Process ID: {row["pid"]}\n  Shell ID: {row["shell_id"]}\n  Config path: {row["config_path"]}')
