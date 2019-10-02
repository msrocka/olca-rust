import os
import platform
import subprocess
import sys

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

OS_MACOS = "macos"
OS_WINDOWS = "windows"
OS_LINUX = "linux"


class Node:

    def __init__(self, path: str, name: str):
        self.path = path
        self.name = name
        self.deps = []


def get_os() -> str:
    ps = platform.system().lower()
    if ps == "darwin":
        return OS_MACOS
    if ps == "windows":
        return OS_WINDOWS
    if ps == "linux":
        return OS_LINUX
    sys.exit("unknown platform: " + ps)


def get_lib_ext() -> str:
    _os = get_os()
    if _os == OS_LINUX:
        return ".so"
    if _os == OS_MACOS:
        return ".dylib"
    if _os == OS_WINDOWS:
        return ".dll"
    sys.exit("unknown os: " + _os)


def as_lib(name: str) -> str:
    _os = get_os()
    prefix = ""
    if _os != OS_WINDOWS:
        if not name.startswith("lib"):
            prefix = "lib"
    return prefix + name + get_lib_ext()


def get_julia_libdir():
    _os = get_os()
    libdir = None
    config = os.path.join(PROJECT_ROOT, "config")
    with open(config, "r", encoding="utf-8") as f:
        libdir_key = _os + "-julia-lib-dir"
        for line in f.readlines():
            parts = line.split("=")
            if len(parts) < 2:
                continue
            key = parts[0].strip()
            if key != libdir_key:
                continue
            libdir = parts[1].strip()
            break
    if libdir is None:
        sys.exit("could not read Julia lib folder for OS=%s from config" % _os)
    return libdir


def get_deps(lib_file: str, libs: list) -> list:
    _os = get_os()
    cmd = None
    if _os == OS_MACOS:
        cmd = ["otool", "-L", lib_file]
    if _os == OS_WINDOWS:
        cmd = ["Dependencies.exe", "-imports", lib_file]
    # TODO linux
    if cmd is None:
        sys.exit("no deps command for os " + _os)

    proc = subprocess.run(cmd, capture_output=True, text=True)
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
            if lib in lib_file:
                continue
            if lib in line:
                deps.append(lib)
    return deps


def get_dep_dag(entry: str) -> Node:
    libdir = get_julia_libdir()
    libs = os.listdir(libdir)
    handled = set()
    root = Node(entry, entry.split(os.path.sep)[-1])
    queue = [root]
    while len(queue) != 0:
        n = queue.pop(0)  # type: Node
        for dep in get_deps(n.path, libs):
            dep_node = Node(os.path.join(libdir, dep), dep)
            n.deps.append(dep_node)
            if dep in handled:
                continue
            handled.add(dep)
            queue.append(dep_node)
    return root


def topo_sort(dag: Node) -> list:
    in_degrees = {}
    dependents = {}
    queue = [dag]
    handled = set()
    while len(queue) != 0:
        n = queue.pop(0)    # type: Node
        if n.name in handled:
            continue
        handled.add(n.name)
        if n.name not in in_degrees:
            in_degrees[n.name] = 0
        for dep in n.deps:
            queue.append(dep)
            if dep.name not in in_degrees:
                in_degrees[dep.name] = 0
            depl = dependents.get(dep.name)
            if depl is None:
                depl = []
                dependents[dep.name] = depl
            depl.append(n.name)
            in_degrees[n.name] = in_degrees[n.name] + 1

    ordered = []
    while len(in_degrees) != 0:

        lib = None
        for _lib, _indeg in in_degrees.items():
            if _indeg == 0:
                lib = _lib
                break
        if lib is None:
            sys.exit("could not calculate dependency order;"
                     + " are there cycles in the dependencies?")

        ordered.append(lib)
        in_degrees.pop(lib)
        depl = dependents.pop(lib, None)
        if depl is None:
            continue
        for dependent in depl:
            in_degrees[dependent] = in_degrees[dependent] - 1

    return ordered


def viz(dep_dag: Node):
    print("digraph g {")
    queue = [dep_dag]
    while len(queue) != 0:
        n = queue.pop(0)
        for dep in n.deps:
            print('  "%s" -> "%s";' % (n.name, dep.name))
            queue.append(dep)
    print("}")


if __name__ == '__main__':
    julia_libdir = get_julia_libdir()
    if julia_libdir is None:
        sys.exit("Could not find the Julia lib folder")
    libs = os.listdir(julia_libdir)
    entry = os.path.join(PROJECT_ROOT, "bin", as_lib("olcar_withumf"))
    # print(get_deps(entry, libs))
    dag = get_dep_dag(entry)
    viz(dag)
    print(topo_sort(dag))
