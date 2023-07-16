plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0b762de7
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:47:04] warning:   ^
[00:47:04] warning: 2 warnings generated.
[00:47:04]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:47:04]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:47:33] error: internal compiler error: librustc/hir/map/mod.rs:520: couldn't find node id 4294967295 in the AST map
[00:47:33] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:599:9
[00:47:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:34] error: aborting due to previous error
[00:47:34] 
[00:47:34] 
[00:47:34] 
[00:47:34] note: the compiler unexpectedly panicked. this is a bug.
[00:47:34] 
[00:47:34] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:34] 
[00:47:34] note: rustc 1.31.0-nightly (a7610b0ff 2018-10-18) running on x86_64-unknown-linux-gnu
[00:47:34] 
[00:47:34] note: compiler flags: -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=2 -C linker=clang -C prefer-dynamic -C linker=clang -C debuginfo=1 -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:47:34] note: some of the compiler flags provided by cargo are hidden
[00:47:34] 
[00:47:34] error: Could not compile `core`.
[00:47:34] warning: build failed, waiting for other jobs to finish...
[00:47:34] warning: build failed, waiting for other jobs to finish...
[00:47:41] error: build failed
[00:47:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:47:41] expected success, got: exit code: 101
[00:47:41] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:47:41] travis_fold:end:stage1-std

[00:47:41] travis_time:end:stage1-std:start=1539830923017471228,finish=1539830972577085049,duration=49559613821

---
travis_time:end:00eef7fd:start=1539830973796494199,finish=1539830973804258711,duration=7764512
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f76c7ec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1367c298
travis_time:start:1367c298
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:058ecd5f
$ dmesg | grep -i kill
