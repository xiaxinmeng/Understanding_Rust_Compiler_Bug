plain
travis_time:end:362b1a5a:start=1553346090645340452,finish=1553346170058743931,duration=79413403479
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:29:54]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:29:54]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:29:55]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:29:55]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:30:15] error: internal compiler error: src/librustc_mir/borrow_check/nll/mod.rs:354: region is not an ReVar: ReStatic
[00:30:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[00:30:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:30:15] error: aborting due to previous error
[00:30:15] 
[00:30:15] 
[00:30:15] 
[00:30:15] note: the compiler unexpectedly panicked. this is a bug.
[00:30:15] 
[00:30:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:30:15] 
[00:30:15] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:30:15] 
[00:30:15] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:30:15] note: some of the compiler flags provided by cargo are hidden
[00:30:15] 
[00:30:15] error: Could not compile `core`.
[00:30:15] 
