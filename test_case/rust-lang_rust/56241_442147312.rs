plain
travis_time:end:0192fd38:start=1543338710199889156,finish=1543338784361235853,duration=74161346697
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:18:40]    Compiling cc v1.0.25
[00:18:40]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:18:40]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:18:40]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:18:42] thread 'main' panicked at 'Went past end of probe sequence', src/libstd/collections/hash/raw/mod.rs:140:9
[00:18:42] 
[00:18:42] error: internal compiler error: unexpected panic
[00:18:42] 
[00:18:42] note: the compiler unexpectedly panicked. this is a bug.
[00:18:42] note: the compiler unexpectedly panicked. this is a bug.
[00:18:42] 
[00:18:42] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:18:42] 
[00:18:42] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:18:42] 
[00:18:42] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:18:42] note: some of the compiler flags provided by cargo are hidden
[00:18:42] 
[00:18:42] error: Could not compile `core`.
[00:18:42] warning: build failed, waiting for other jobs to finish...
---
184272 ./obj/build/cache/2018-10-30
151428 ./src/tools/clang
150252 ./obj/build/bootstrap/debug/incremental
134660 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134656 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f72g3h9t3r-10njel7-18zp7vy2v2s65
127408 ./.git/modules/src
115360 ./src/llvm/test/CodeGen
107888 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
106852 ./src/tools/lldb
