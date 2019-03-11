@echo off
cargo clean
del /s /q /f bin\*.lib
del /s /q /f bin\*.exp
del /s /q /f bin\*.def

xcopy /y windefs\libumfpack.def bin
xcopy /y windefs\libopenblas64_.def bin
cd bin
lib /def:libumfpack.def /out:libumfpack.lib /machine:X64
lib /def:libopenblas64_.def /out:libopenblas64_.lib /machine:X64
cd ..

cargo build --release
xcopy /y target\release\olcar.dll bin
