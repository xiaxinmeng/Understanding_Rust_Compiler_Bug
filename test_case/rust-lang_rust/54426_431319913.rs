plain
[00:19:31]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:19:31]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:19:32]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:19:32]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:20:01] error: internal compiler error: librustc_codegen_llvm/context.rs:476: failed to get layout for `iter::Rev<I>`: the type `I` has an unknown layout
[00:20:01] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:599:9
[00:20:01] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:20:01] error: aborting due to previous error
[00:20:01] 
[00:20:01] 
[00:20:01] 
[00:20:01] note: the compiler unexpectedly panicked. this is a bug.
[00:20:01] 
[00:20:01] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:20:01] 
[00:20:01] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:20:01] 
[00:20:01] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:20:01] note: some of the compiler flags provided by cargo are hidden
[00:20:01] 
[00:20:01] error: Could not compile `core`.
[00:20:01] 
[00:20:01] 
[00:20:01] To learn more, run the command again with --verbose.
[00:20:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:01] expected success, got: exit code: 101
[00:20:01] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:20:01] travis_fold:end:stage1-std

[00:20:01] travis_time:end:stage1-std:start=1539945129841177962,finish=1539945171310421889,duration=41469243927


[00:20:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:01] Build completed unsuccessfully in 0:16:11
[00:20:01] Makefile:28: recipe for target 'all' failed
[00:20:01] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0683cbe6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
