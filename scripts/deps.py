import os
import platform
import subprocess
import sys

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


class Node:

    def __init__(self, path: str, name: str):
        self.path = path
        self.name = name
        self.deps = []


def get_julia_libdir():
    os_prefix = None
    ps = platform.system()
    if ps == 'Darwin':
        os_prefix = 'macos'
    if os_prefix is None:
        sys.exit("unknown platform: " + ps)

    libdir = None
    config = os.path.join(PROJECT_ROOT, "config")
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
    entry = os.path.join(PROJECT_ROOT, "bin", "libolcar_withumf.dylib")
    # print(get_deps(entry, libs))
    dag = get_dep_dag(entry)
    viz(dag)