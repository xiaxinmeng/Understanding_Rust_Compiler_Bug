
test fs::tests::recursive_rmdir ... ok


command did not execute successfully: "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\obj\\build\\i686-pc-windows-gnu\\stage0/bin\\cargo.exe" "test" "-j" "8" "--target" "i686-pc-windows-gnu" "--release" "--manifest-path" "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\src/rustc/std_shim\\Cargo.toml" "--features" " jemalloc" "-p" "std_shim" "-p" "alloc" "-p" "alloc_system" "-p" "build_helper" "-p" "collections" "-p" "core" "-p" "libc" "-p" "panic_abort" "-p" "panic_unwind" "-p" "rand" "-p" "rustc_unicode" "-p" "std" "-p" "unwind"
expected success, got: exit code: 3221225477


thread '<unnamed>' panicked at 'explicit panic', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\libstd\io\buffered.rs:1109
note: Run with `RUST_BACKTRACE=1` for a backtrace.
fatal runtime error: failed to initiate panic, error 5
