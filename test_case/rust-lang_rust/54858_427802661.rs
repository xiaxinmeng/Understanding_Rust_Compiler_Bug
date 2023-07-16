plain
[00:19:49]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:19:50]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:19:50]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:19:51]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:20:16] error: internal compiler error: librustc/infer/canonical/canonicalizer.rs:175: unexpected region in query response: `ReErased`
[00:20:16] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:599:9
[00:20:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:20:16] error: aborting due to previous error
[00:20:16] 
[00:20:16] 
[00:20:16] 
[00:20:16] note: the compiler unexpectedly panicked. this is a bug.
[00:20:16] 
[00:20:16] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:20:16] 
[00:20:16] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:20:16] 
[00:20:16] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:20:16] note: some of the compiler flags provided by cargo are hidden
[00:20:16] 
[00:20:16] error: Could not compile `core`.
[00:20:16] 
[00:20:16] 
[00:20:16] To learn more, run the command again with --verbose.
[00:20:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:16] expected success, got: exit code: 101
[00:20:16] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:20:16] travis_fold:end:stage1-std

[00:20:16] travis_time:end:stage1-std:start=1538998851388341082,finish=1538998888490085661,duration=37101744579


[00:20:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:16] Build completed unsuccessfully in 0:15:49
[00:20:16] make: *** [all] Error 1
[00:20:16] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0613bab3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
