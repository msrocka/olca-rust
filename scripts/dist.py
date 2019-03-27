import os
import platform

from shutil import copyfile, make_archive


def main():
    system = platform.system()
    if system not in ["Windows", "Darwin"]:
        print("Unknown system", system)
        return

    os.makedirs("./target/dist/wi_umfpack", exist_ok=True)
    os.makedirs("./target/dist/wo_umfpack", exist_ok=True)
    suffix = get_suffix(system)

    wi_umf_libs = get_all_libs(system)
    dest = "./target/dist/wi_umfpack/"
    for lib in wi_umf_libs:
        copyfile("./bin/" + lib, dest + lib)
    make_archive('./target/dist/wi_umfpack_' + suffix, 'zip',
                 './target/dist/wi_umfpack')

    wo_umf_libs = get_blas_libs(system)
    dest = "./target/dist/wo_umfpack/"
    for lib in wo_umf_libs:
        copyfile("./bin/" + lib, dest + lib)
    make_archive('./target/dist/wo_umfpack_' + suffix, 'zip',
                 './target/dist/wo_umfpack')


def get_suffix(system):
    if system == "Windows":
        return "win_x64"
    if system == "Darwin":
        return "macos_x64"
    return "none"


def get_all_libs(system):
    if system == "Windows":
        return [
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
            "libcolamd.dll",
        ]
    if system == "Darwin":
        return [
            "libgcc_s.1.dylib",
            "libquadmath.0.dylib",
            "libgfortran.4.dylib",
            "libopenblas64_.dylib",
            "libsuitesparseconfig.dylib",
            "libamd.dylib",
            "libcamd.dylib",
            "libcolamd.dylib",
            "libccolamd.dylib",
            "libcholmod.dylib",
            "libumfpack.dylib",
            "libolcar_withumf.dylib",
        ]


def get_blas_libs(system):
    if system == "Windows":
        return [
            "olcar.dll",
            "libopenblas64_.dll",
            "libgfortran-3.dll",
            "libquadmath-0.dll",
            "libgcc_s_seh-1.dll",
            "libwinpthread-1.dll",
        ]
    if system == "Darwin":
        return [
            "libgcc_s.1.dylib",
            "libquadmath.0.dylib",
            "libgfortran.4.dylib",
            "libopenblas64_.dylib",
            "libolcar.dylib",
        ]


if __name__ == "__main__":
    main()
