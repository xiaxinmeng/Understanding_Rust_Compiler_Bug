
Testing libstd stage2 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
   Compiling rand v0.0.0 (file:///C:/projects/rust/src/librand)
   Compiling alloc v0.0.0 (file:///C:/projects/rust/src/liballoc)
   Compiling collections v0.0.0 (file:///C:/projects/rust/src/libcollections)
error: failed to remove: C:\projects\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\libcollections.rlib
Build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "test" "-j" "2" "--target" "x86_64-pc-windows-msvc" "--release" "--locked" "--manifest-path" "C:\\projects\\rust\\src/libstd\\Cargo.toml" "--features" "panic-unwind backtrace" "-p" "std:0.0.0" "-p" "compiler_builtins:0.0.0" "-p" "std_unicode:0.0.0" "-p" "alloc:0.0.0" "-p" "collections:0.0.0" "-p" "rand:0.0.0" "-p" "rustc_lsan:0.0.0" "-p" "rustc_asan:0.0.0" "-p" "panic_abort:0.0.0" "-p" "alloc_system:0.0.0" "-p" "unwind:0.0.0" "-p" "core:0.0.0" "-p" "rustc_tsan:0.0.0" "-p" "libc:0.0.0" "-p" "rustc_msan:0.0.0" "--"
expected success, got: exit code: 101
