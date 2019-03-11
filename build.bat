@echo off

xcopy /y windefs\umfpack.def bin
xcopy /y windefs\openblas.def bin
cd bin
lib /def:umfpack.def /out:umfpack.lib /machine:X64
lib /def:openblas.def /out:openblas.lib /machine:X64
cd ..

cargo build --release
xcopy /y target\release\olcar.dll bin
