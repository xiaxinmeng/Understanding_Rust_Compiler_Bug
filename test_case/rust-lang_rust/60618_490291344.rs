plain
travis_time:end:07d2fecc:start=1557270259665799738,finish=1557270346337693066,duration=86671893328
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:26:11]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:26:11]    Compiling libc v0.2.54
[00:26:11]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:26:12]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:26:12] thread 'rustc' panicked at 'assertion failed: src.len() > 0', src/libsyntax/ext/tt/transcribe.rs:74:5
[00:26:12] 
[00:26:12] error: internal compiler error: unexpected panic
[00:26:12] 
[00:26:12] note: the compiler unexpectedly panicked. this is a bug.
[00:26:12] note: the compiler unexpectedly panicked. this is a bug.
[00:26:12] 
[00:26:12] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:26:12] 
[00:26:12] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[00:26:12] 
[00:26:12] note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:26:12] note: some of the compiler flags provided by cargo are hidden
[00:26:12] 
[00:26:12] error: Could not compile `core`.
[00:26:12] warning: build failed, waiting for other jobs to finish...
