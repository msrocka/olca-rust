# jumf
`jumf` is an experimental library that provides Java bindings to
[UMFPACK](https://en.wikipedia.org/wiki/UMFPACK) where the
[JNI](https://en.wikipedia.org/wiki/Java_Native_Interface) glue code is written
in [Rust](https://www.rust-lang.org/). If this works well on Windows, macOS, and
Linux we may merge this into the [openLCA core](https://github.com/GreenDelta/olca-modules)
to call into our native high performance math libraries.

## Getting UMFPACK
An easy way to get the compiled UMFPACK libraries together with an high
performance BLAS implementation is to take them from the
[Julia](https://julialang.org/) binaries.

### Windows
On Windows you can use [Dependency Walker](http://www.dependencywalker.com/) to
analyze the library dependencies. You should get the following dependecy tree:

```
-> libumfpack.dll
  -> libopenblas64_.dll
	-> libgfortran-3.dll
	  -> libquadmath-0.dll
	    -> libgcc_s_seh-1.dll
          -> libwinpthread-1.dll
  -> libamd.dll
    -> libsuitesparseconfig.dll
  -> libcholmod.dll
    -> libopenblas64_.dll
    -> libsuitesparseconfig.dll
	-> libccolamd.dll 
       -> libsuitesparseconfig.dll
    -> libcamd.dll
       -> libsuitesparseconfig.dll
       -> libamd.dll
         -> libsuitesparseconfig.dll
    -> libcolamd.dll
       -> libsuitesparseconfig.dll
```