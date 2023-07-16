plain
travis_time:end:34dacb3a:start=1551256837683291170,finish=1551256838556473904,duration=873182734
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:27:22]    |
[00:27:22] 71 | #[allow_internal_unstable]
[00:27:22]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:27:22] 
[00:27:31] error: internal compiler error: src/librustc/ty/relate.rs:703: impossible case reached: can't relate: '_#0r with Type(RustaceansAreAwesome)
[00:27:31] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:620:9
[00:27:31] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:27:31] error: aborting due to previous error
[00:27:31] 
[00:27:31] 
[00:27:31] 
[00:27:31] note: the compiler unexpectedly panicked. this is a bug.
[00:27:31] 
[00:27:31] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:27:31] 
[00:27:31] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[00:27:31] 
[00:27:31] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:27:31] note: some of the compiler flags provided by cargo are hidden
[00:27:31] 
[00:27:31] error: Could not compile `rustc`.
[00:27:31] warning: build failed, waiting for other jobs to finish...
