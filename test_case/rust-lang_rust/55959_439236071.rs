plain
travis_time:end:08df50cd:start=1542325696006758539,finish=1542325698385676566,duration=2378918027
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:21:29] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:21:29] 
[00:21:29] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:21:29] 
[00:21:29] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:21:29] note: some of the compiler flags provided by cargo are hidden
[00:21:29] 
[00:21:29] error: Could not compile `core`.
[00:21:29] 
[00:21:29] 
[00:21:29] To learn more, run the command again with --verbose.
[00:21:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:29] expected success, got: exit code: 101
[00:21:29] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:21:29] travis_fold:end:stage1-std

[00:21:29] travis_time:end:stage1-std:start=1542326961861359225,finish=1542326997285564274,duration=35424205049

