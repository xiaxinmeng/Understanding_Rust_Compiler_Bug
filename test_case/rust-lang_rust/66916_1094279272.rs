batch
@echo off

call emsdk/emsdk_env >NUL 2>NUL
set EMCC_CFLAGS=-s ERROR_ON_UNDEFINED_SYMBOLS=0 --no-entry

cmd /k
