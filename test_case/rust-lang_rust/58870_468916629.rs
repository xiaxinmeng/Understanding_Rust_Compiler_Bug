plain
travis_time:end:0fff643e:start=1551528449638587671,finish=1551528450524937519,duration=886349848
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:26:25]    Compiling env_logger v0.5.13
[00:26:29]    Compiling flate2 v1.0.6
[00:26:29]    Compiling backtrace v0.3.11
[00:26:35]    Compiling tempfile v3.0.5
[00:26:36] error: internal compiler error: src/librustc_codegen_llvm/intrinsic.rs:709: broken spin_loop_hint
[00:26:36] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:620:9
[00:26:36] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:26:36] error: aborting due to previous error
[00:26:36] 
[00:26:36] 
[00:26:36] 
[00:26:36] note: the compiler unexpectedly panicked. this is a bug.
[00:26:36] 
[00:26:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:26:36] 
[00:26:36] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[00:26:36] 
[00:26:36] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:26:36] note: some of the compiler flags provided by cargo are hidden
[00:26:36] 
[00:26:36] error: Could not compile `parking_lot_core`.
[00:26:36] warning: build failed, waiting for other jobs to finish...
---
181352 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
156416 ./src/llvm-project/clang
156004 ./obj/build/bootstrap/debug/incremental
141232 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn
141228 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn/s-f9yyeooc9d-1jcjbmg-3r437gq9en0c0
139296 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
136416 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
123628 ./src/llvm-project/llvm/test/CodeGen
108532 ./src/llvm-project/lldb
