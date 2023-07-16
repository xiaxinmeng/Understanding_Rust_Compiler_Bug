
error: Could not remove file: C:\projects\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps\libcompiler_builtins-7054340c38d0eb75.rlib.
Caused by:
  Access is denied. (os error 5)
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "test" "--target" "x86_64-pc-windows-msvc" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler" "--manifest-path" "C:\\projects\\rust\\src/libstd/Cargo.toml" "-p" "std:0.0.0" "-p" "core:0.0.0" "-p" "collections:0.0.0" "-p" "panic_abort:0.0.0" "-p" "unwind:0.0.0" "-p" "std_unicode:0.0.0" "-p" "alloc_system:0.0.0" "-p" "libc:0.0.0" "-p" "alloc:0.0.0" "-p" "rand:0.0.0" "-p" "compiler_builtins:0.0.0" "--"
expected success, got: exit code: 101

