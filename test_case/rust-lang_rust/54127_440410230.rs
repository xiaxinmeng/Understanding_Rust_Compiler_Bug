plain
travis_time:end:12000015:start=1542742727978586881,finish=1542742729155859064,duration=1177272183
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:20:31]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:20:31]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:20:32]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:20:32]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:20:37] error: internal compiler error: librustc/hir/map/mod.rs:235: local_def_id: no entry for `118816`, which has a map of `None`
[00:20:37] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:600:9
[00:20:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:20:38] error: aborting due to previous error
[00:20:38] 
[00:20:38] 
[00:20:38] 
[00:20:38] note: the compiler unexpectedly panicked. this is a bug.
[00:20:38] 
[00:20:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:20:38] 
[00:20:38] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:20:38] 
[00:20:38] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:20:38] note: some of the compiler flags provided by cargo are hidden
[00:20:38] 
[00:20:38] error: Could not compile `core`.
[00:20:38] 
