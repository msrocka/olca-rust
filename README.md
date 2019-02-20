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

We put these libraries into the `jumf/rust/jumf/bin` folder. In order to build
the Windows library we also need to
[generate a lib-file](https://stackoverflow.com/a/16127548/599575) from the
`libumfpack.dll` library:

1. Add the `dumpbin` and `lib` tools from MSVC 2017 build tools (which are anyhow
   required for the Rust compiler) to your path (something like
   `C:\Program Files (x86)\Microsoft Visual Studio\2017\BuildTools\VC\Tools\MSVC\14.16.27023\bin\Hostx64\x64`)
2. run `dumpbin /EXPORTS libumfpack.dll > libumfpack.exports` in the bin folder
3. Paste the names of the functions from `libumfpack.exports` into a new
   `libumfpack.def` file. Add a line with the word `EXPORTS` at the top of this
   file. (the `libumfpack.def` file is versioned the next steps are part of the
   `build.bat` script)
4. Generate the `lib` file: lib /def:libumfpack.def /out:libumfpack.lib (this
   will also generate an `libumfpack.exp` file)


### Linux

* the library name of UMFPACK is `libumfpack.so` but the linking name is `umfpack`:
  `#[link(name = "umfpack"]`

The folder with the shared libraries needs to be in the `LD_LIBRARY_PATH`. When this
is the case the tests run, e.g.:

```bash
export LD_LIBRARY_PATH=/path/to/bin/folder

## and then the Maven test works:
mvn test
```

see also: https://stackoverflow.com/a/7284911

Eclipse should add the folder with the executable launcher to the `LD_LIBRARY_PATH`:

  
  
