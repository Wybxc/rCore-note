"""动态修改每个用户程序的加载地址，依次偏移 0x20000 字节。
"""
import os

base_address = 0x80400000
step = 0x20000
linker = "src/linker.ld"

app_id = 0
apps = os.listdir("src/bin")
apps.sort()
for app in apps:
    app = app[: app.find(".")]

    lines = []
    lines_before = []
    with open(linker, "r") as f:
        for line in f.readlines():
            lines_before.append(line)
            line = line.replace(hex(base_address), hex(base_address + step * app_id))
            lines.append(line)
    with open(linker, "w+") as f:
        f.writelines(lines)

    mode = os.getenv("RCORE_MODE", "debug")
    mode_arg = "--release" if mode == "release" else ""
    os.system(f"cargo build --bin {app} {mode_arg}")

    print(
        f"[build.py] application {app} start with address 0x{base_address + step * app_id:x}"
    )
    with open(linker, "w+") as f:
        f.writelines(lines_before)

    app_id = app_id + 1
