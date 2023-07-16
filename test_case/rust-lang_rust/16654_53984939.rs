
$ PATH="/c/repo/mingw-w64-rust-git/src/build-x86_64/x86_64-w64-mingw32/stage1/bin:/mingw64/bin:/usr/local/bin:/usr/bin:/bin:/opt/bin:/c/windows/system32:/c/windows:/c/windows/System32/WindowsPowerShell/v1.0:/usr/bin/vendor_perl:/usr/bin/core_perl:x86_64-w64-mingw32/stage1/bin"   x86_64-w64-mingw32/stage1/bin/rustc.exe --cfg stage1  -O --cfg rtopt --cfg debug -C prefer-dynamic --target=x86_64-w64-mingw32  -D warnings -L "x86_64-w64-mingw32/rt" -L "C:\msys64\mingw64/lib" -L ""  --out-dir x86_64-w64-mingw32/stage1/bin/rustlib/x86_64-w64-mingw32/lib -C extra-filename=-4e7c5e5c /c/repo/mingw-w64-rust-git/src/rust/src/libcore/lib.rs
LLVM ERROR: Segmented stacks not supported on this platform.
Segmentation fault
