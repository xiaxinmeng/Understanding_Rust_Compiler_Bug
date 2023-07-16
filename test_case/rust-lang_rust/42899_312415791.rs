
error: failed to run custom build command for `profiler_builtins v0.0.0 (file:///C:/projects/rust/src/libprofiler_builtins)`
process didn't exit successfully: `C:\projects\rust\build\x86_64-pc-windows-msvc\stage0-std\release\build\profiler_builtins-7f008c34aad7d625\build-script-build` (exit code: 101)
...
running: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\cl.exe" "/nologo" "/MT" "/O2" "/Zl" "/Dstrdup=_strdup" "/Dopen=_open" "/Dfdopen=_fdopen" "/FoC:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\profiler_builtins-bcaf90ff1dc76203\\out\\../libcompiler_builtins/compiler-rt/lib/profile\\WindowsMMap.o" "/c" "../libcompiler_builtins/compiler-rt/lib/profile\\WindowsMMap.c"
WindowsMMap.c
../libcompiler_builtins/compiler-rt/lib/profile\WindowsMMap.c(42): error C2065: 'DWORD': undeclared identifier
../libcompiler_builtins/compiler-rt/lib/profile\WindowsMMap.c(42): error C2146: syntax error: missing ';' before identifier 'flProtect'
../libcompiler_builtins/compiler-rt/lib/profile\WindowsMMap.c(42): error C2065: 'flProtect': undeclared identifier
../libcompiler_builtins/compiler-rt/lib/profile\WindowsMMap.c(45): error C2065: 'flProtect': undeclared identifier
...
