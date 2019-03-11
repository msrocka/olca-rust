@echo off

xcopy /y windefs\libumfpack.def bin
cd bin
lib /def:libumfpack.def /out:umfpack.lib /machine:X64
cd ..

cargo build --release
xcopy /y target\release\olcar.dll bin
