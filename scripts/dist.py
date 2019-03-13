import os
import platform

from shutil import copyfile


def main():
    os.makedirs("./target/dist/wi_umfpack", exist_ok=True)
    os.makedirs("./target/dist/wo_umfpack", exist_ok=True)
    system = platform.system()
    if system == "Windows":
        copy_winlibs()


def copy_winlibs():
    wi_umf_libs = [
        "olcar_withumf.dll",
        "libopenblas64_.dll",
        "libgfortran-3.dll",
        "libquadmath-0.dll",
        "libgcc_s_seh-1.dll",
        "libwinpthread-1.dll",
        "libumfpack.dll",
        "libsuitesparseconfig.dll",
        "libamd.dll",
        "libcholmod.dll",
        "libccolamd.dll",
        "libcamd.dll",
        "libcolamd.dll"
    ]
    dest = "./target/dist/wi_umfpack/"
    for lib in wi_umf_libs:
        copyfile("./bin/" + lib, dest + lib)

    wo_umf_libs = [
        "olcar.dll",
        "libopenblas64_.dll",
        "libgfortran-3.dll",
        "libquadmath-0.dll",
        "libgcc_s_seh-1.dll",
        "libwinpthread-1.dll",
    ]
    dest = "./target/dist/wo_umfpack/"
    for lib in wo_umf_libs:
        copyfile("./bin/" + lib, dest + lib)


if __name__ == "__main__":
    main()
