
Building stage1 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 656.99 secs
   Compiling build_helper v0.1.0 (file:///C:/bot/slave/auto-win-msvc-64-cargotest/build/src/build_helper)
   Compiling core v0.0.0 (file:///C:/bot/slave/auto-win-msvc-64-cargotest/build/src/libcore)
   Compiling gcc v0.3.35
   Compiling unwind v0.0.0 (file:///C:/bot/slave/auto-win-msvc-64-cargotest/build/src/libunwind)
   Compiling libc v0.0.0 (file:///C:/bot/slave/auto-win-msvc-64-cargotest/build/src/rustc/libc_shim)
Build failed, waiting for other jobs to finish...
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

thread 'rustc' panicked at 'should never swap if not fully enabled', C:\bot\slave\auto-win-msvc-64-cargotest\build\src\librustc\dep_graph\thread.rs:108
note: Run with `RUST_BACKTRACE=1` for a backtrace.
