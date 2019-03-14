# olca-rust
`olca-rust` is an experimental project the provides 
[JNI bindings](https://en.wikipedia.org/wiki/Java_Native_Interface) for the
native math libraries that are used in [openLCA](https://github.com/GreenDelta/olca-app)
where the glue code is written in [Rust](https://www.rust-lang.org/). If this
works well on Windows, macOS, and Linux we may merge this into the
[openLCA core](https://github.com/GreenDelta/olca-modules) to call into native
code.

We currently link dynamically to [OpenBLAS](https://github.com/xianyi/OpenBLAS)
and [UMFPACK](https://github.com/PetterS/SuiteSparse) and we take pre-compiled
versions of these libraries for the respective platforms directly from the
[Julia](https://julialang.org/) distribution packages.

Calling functions into these libraries via JNI is not fun but Rust can make life
a lot easier here as it supports among others the following things:

* a [standard project layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
  with [modern tools](https://www.rust-lang.org/tools) that work exactly the
  same on all platforms
* [conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html): 
  we configure links to platform specific libraries and symbols (function names)
  directly in the code
* [modules](https://doc.rust-lang.org/beta/book/ch07-02-modules-and-use-to-control-scope-and-privacy.html):
  we can hide the details behind a platform independent API in a module
* [documentation tool](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html):
  just run `cargo doc` or `cargo doc --document-private-items --no-deps`


## Project layout
This project is a standard Cargo package. It expects to find the native
libraries (see below) in the `bin` folder of this project. There is a build
script (`build.bat` for Windows and `build.sh` for Linux and macOS) which
creates the libraries with the JNI bindings (`olcar.{dll|so|dylib}` with BLAS
& LAPACK bindings; `olcar_withumf.{dll|so|dylib}` with additional UMFPACK
bindings). The `java` folder contains a Maven project that tests the JNI
bindings against the interface of the
[openLCA modules](https://github.com/GreenDelta/olca-modules). Finally, there
is a script `scripts/dist.py` that creates the distribution packages for
openLCA.

```bash
cd olca-rust
# copy the native libraries into the `bin` folder
./build.sh               # build the JNI bindings
mvn -f java/pom.xml test # run the tests
python scripts/dist.py   # create the distribution packages
```

## Libraries
As said above, we directly take the compiled libraries from the respective
[Julia](https://julialang.org/) distribution packages.

### Windows
On Windows you can use [Dependency Walker](http://www.dependencywalker.com/) to
analyze the library dependencies. The following lists the library dependencies

```
# OpenBLAS
-> libopenblas64_.dll
 -> libgfortran-3.dll
  -> libquadmath-0.dll
   -> libgcc_s_seh-1.dll
    -> libwinpthread-1.dll

# UMFPACK
-> libumfpack.dll
 -> libcholmod.dll
  -> libccolamd.dll 
   -> libcolamd.dll
    -> libcamd.dll
     -> libamd.dll
      -> libsuitesparseconfig.dll
       + OpenBLAS
```

On Windows, we also need to [generate a lib-file](https://stackoverflow.com/a/16127548/599575)
for each library we want to link again. This is done automatically from the
definition files in the `windefs` in the `build.bat` script but this requires
that the `lib` tool from the MSVC 2017 build tools (which are anyhow required
for the Rust compiler) is in your `PATH` (e.g. something like this:
`C:\Program Files (x86)\Microsoft Visual Studio\2017\BuildTools\VC\Tools\MSVC\14.16.27023\bin\Hostx64\x64`)


### Linux

**todo**
The folder with the shared libraries needs to be in the `LD_LIBRARY_PATH`. When this
is the case the tests run, e.g.:

```bash
export LD_LIBRARY_PATH=/path/to/bin/folder

## and then the Maven test works:
mvn test
```

see also: https://stackoverflow.com/a/7284911

Eclipse should add the folder with the executable launcher to the `LD_LIBRARY_PATH`:
https://eclipsesource.com/blogs/2012/08/18/using-shared-libraries-with-eclipse-rcp/

?

### macOS

**todo**
  
  
