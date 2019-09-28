import os
import os.path as path
import platform
import subprocess
import sys

PROJECT_ROOT = path.dirname(path.dirname(path.abspath(__file__)))


class Node:

    def __init__(self, lib: str):
        self.lib = lib
        self.deps = []


def get_julia_libdir():
    os_prefix = None
    ps = platform.system()
    if ps == 'Darwin':
        os_prefix = 'macos'
    if os_prefix is None:
        sys.exit("unknown platform: " + ps)

    libdir = None
    config = path.join(PROJECT_ROOT, "config")
    with open(config, "r", encoding="utf-8") as f:
        libdir_key = os_prefix + "-julia-lib-dir"
        for line in f.readlines():
            parts = line.split("=")
            if len(parts) < 2:
                continue
            key = parts[0].strip()
            if key != libdir_key:
                continue
            libdir = parts[1].strip()
            break
    return libdir


def get_deps(lib_file: str, libs: list) -> list:
    # TODO: platform calls here
    proc = subprocess.run(["otool", "-L", lib_file],
                          capture_output=True, text=True)
    out = None
    if proc.stdout is not None:
        out = proc.stdout
    elif proc.stderr is not None:
        out = proc.stderr
    if out is None:
        return []
    deps = []
    for line in out.splitlines():
        for lib in libs:
            if lib in line:
                deps.append(lib)
    return deps


if __name__ == '__main__':
    julia_libdir = get_julia_libdir()
    if julia_libdir is None:
        sys.exit("Could not find the Julia lib folder")
    libs = os.listdir(julia_libdir)
    entry = path.join(PROJECT_ROOT, "bin", "libolcar.dylib")
    print(get_deps(entry, libs))
