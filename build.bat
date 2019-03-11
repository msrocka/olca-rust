@echo off

cd bin
lib /def:libumfpack.def /out:libumfpack.lib /machine:X64
cd ..

cargo build --release
xcopy /y target\release\olcar.dll bin
