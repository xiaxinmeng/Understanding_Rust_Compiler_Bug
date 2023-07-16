plain
travis_time:end:177a15fe:start=1542392089429619845,finish=1542392142816546067,duration=53386926222
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:22:12]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:22:12]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:22:12]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:22:13]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:22:34] error: internal compiler error: librustc_traits/type_op.rs:148: user ty should have a substs
[00:22:34] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:600:9
[00:22:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:22:34] error: aborting due to previous error
[00:22:34] 
[00:22:34] 
[00:22:34] 
[00:22:34] note: the compiler unexpectedly panicked. this is a bug.
[00:22:34] 
[00:22:34] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:22:34] 
[00:22:34] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:22:34] 
[00:22:34] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:22:34] note: some of the compiler flags provided by cargo are hidden
[00:22:34] 
[00:22:34] error: Could not compile `core`.
[00:22:34] 
[00:22:34] 
[00:22:34] To learn more, run the command again with --verbose.
[00:22:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:22:34] expected success, got: exit code: 101
[00:22:34] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:22:34] travis_fold:end:stage1-std

[00:22:34] travis_time:end:stage1-std:start=1542393472540585477,finish=1542393506785273200,duration=34244687723


[00:22:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:34] Build completed unsuccessfully in 0:18:10
[00:22:34] Makefile:28: recipe for target 'all' failed
[00:22:34] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16816dbb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 16 18:38:27 UTC 2018
