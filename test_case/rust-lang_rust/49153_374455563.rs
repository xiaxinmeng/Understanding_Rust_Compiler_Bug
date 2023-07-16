
Building stage1 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
   Compiling cc v1.0.6
   Compiling core v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libcore)
   Compiling cfg-if v0.1.2
   Compiling unwind v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libunwind)
   Compiling filetime v0.1.15
   Compiling build_helper v0.1.0 (file:///C:/Users/cyber/Rust/rust/src/build_helper)
   Compiling compiler_builtins v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/rustc/compiler_builtins_shim)
   Compiling std v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libstd)
   Compiling alloc_jemalloc v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/liballoc_jemalloc)
   Compiling libc v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/rustc/libc_shim)
   Compiling std_unicode v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libstd_unicode)
   Compiling panic_abort v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libpanic_abort)
   Compiling alloc v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/liballoc)
   Compiling panic_unwind v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libpanic_unwind)
   Compiling alloc_system v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/liballoc_system)
[0m[38;5;9m[1merror: internal compiler error[0m[0m[38;5;15m[1m: librustc\mir\interpret\mod.rs:184: alloc id without corresponding allocation: 39[0m

thread 'rustc' panicked at 'Box<Any>', librustc_errors\lib.rs:543:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-dev running on x86_64-pc-windows-msvc

note: compiler flags: -Z force-unstable-if-unmarked -C debug-assertions=off -C overflow-checks=on -C incremental -C prefer-dynamic -C debug-assertions=y -C codegen-units=8 -C target-feature=+crt-static --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `panic_unwind`.

Caused by:
  process didn't exit successfully: `C:\Users\cyber\Rust\rust\build\bootstrap/debug/rustc --crate-name panic_unwind libpanic_unwind\lib.rs --error-format json --crate-type lib --emit=dep-info,link -C debug-assertions=off -C overflow-checks=on -C metadata=730d0c4f6ad8f657 -C extra-filename=-730d0c4f6ad8f657 --out-dir C:\Users\cyber\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\debug\deps --target x86_64-pc-windows-msvc -C incremental=C:\Users\cyber\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\debug\incremental -L dependency=C:\Users\cyber\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\debug\deps -L dependency=C:\Users\cyber\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\debug\deps --extern core=C:\Users\cyber\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\debug\deps\libcore-814295c4539117a2.rlib --extern alloc=C:\Users\cyber\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\debug\deps\liballoc-0ac88ae22893c7ce.rlib --extern libc=C:\Users\cyber\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\debug\deps\liblibc-766a10703183d997.rlib --extern unwind=C:\Users\cyber\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\debug\deps\libunwind-75ec301683f1f6ba.rlib` (exit code: 101)
warning: build failed, waiting for other jobs to finish...
error: build failed
thread 'main' panicked at 'command did not execute successfully: "C:/Users/cyber/.cargo/bin/cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-j" "8" "--features" "panic-unwind debug-jemalloc backtrace" "--manifest-path" "C:\\Users\\cyber\\Rust\\rust\\src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', bootstrap\compile.rs:1096:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: C:\Users\cyber\Rust\rust\build\bootstrap\debug\bootstrap build
Build completed unsuccessfully in 0:25:34
