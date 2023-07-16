plain
[00:40:58]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:41:01]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:41:05]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:41:05]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:41:35] error: internal compiler error: librustc/ty/layout.rs:992: impossible case reached
[00:41:35] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
[00:41:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:41:35] error: aborting due to previous error
[00:41:35] 
[00:41:35] 
[00:41:35] 
[00:41:35] note: the compiler unexpectedly panicked. this is a bug.
[00:41:35] 
[00:41:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:41:35] 
[00:41:35] note: rustc 1.27.0-nightly (d14b569c4 2018-05-02) running on x86_64-unknown-linux-gnu
[00:41:35] 
[00:41:35] note: compiler flags: -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=3 -C prefer-dynamic -C linker=cc -C debuginfo=1 -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:41:35] 
[00:41:35] note: some of the compiler flags provided by cargo are hidden
[00:41:35] [RUSTC-TIMING] core test:false 37.544
[00:41:35] error: Could not compile `core`.
[00:41:35] 
[00:41:35] Caused by:
[00:41:35] Caused by:
[00:41:35]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=7b4fdee1d7503d3d -C extra-filename=-7b4fdee1d7503d3d --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps --target i586-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps` (exit code: 101)
[00:41:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i586-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:41:35] expected success, got: exit code: 101
[00:41:35] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:41:35] travis_fold:end:stage2-std

[00:41:35] travis_time:end:stage2-std:start=1525280392665311277,finish=1525280430627959368,duration=37962648091

