plain
travis_time:end:2a2b9b48:start=1558624961865252143,finish=1558625049182062914,duration=87316810771
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:24:00]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:24:00]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:24:00]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:24:00]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:24:05] error: internal compiler error: src/librustc/hir/def.rs:271: attempted .def_id() on invalid def: Local(NodeId(13246))
[00:24:05] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:635:9
[00:24:05] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:24:06] error: aborting due to previous error
[00:24:06] 
[00:24:06] 
[00:24:06] 
[00:24:06] note: the compiler unexpectedly panicked. this is a bug.
[00:24:06] 
[00:24:06] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:24:06] 
[00:24:06] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:24:06] 
[00:24:06] note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:24:06] note: some of the compiler flags provided by cargo are hidden
[00:24:06] 
[00:24:06] error: Could not compile `core`.
[00:24:06] 
