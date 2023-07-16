plain
travis_time:end:055d534f:start=1553804803168014353,finish=1553804804075804180,duration=907789827
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:27:25]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:27:26]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:27:26]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:27:27]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:27:28] thread 'rustc' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:997:5
[00:27:28] 
[00:27:28] error: internal compiler error: unexpected panic
[00:27:28] 
[00:27:28] note: the compiler unexpectedly panicked. this is a bug.
[00:27:28] note: the compiler unexpectedly panicked. this is a bug.
[00:27:28] 
[00:27:28] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:27:28] 
[00:27:28] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:27:28] 
[00:27:28] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:27:28] note: some of the compiler flags provided by cargo are hidden
[00:27:28] 
[00:27:28] error: Could not compile `core`.
[00:27:28] warning: build failed, waiting for other jobs to finish...
