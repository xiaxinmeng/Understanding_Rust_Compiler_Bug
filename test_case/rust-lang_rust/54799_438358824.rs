plain
travis_time:end:081d6492:start=1542128310448860848,finish=1542129313293672947,duration=1002844812099
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:09:14]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:09:17] error: build failed
[00:09:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:09:17] expected success, got: exit code: 101
[00:09:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:09:17] travis_fold:end:stage0-rustc

[00:09:17] travis_time:end:stage0-rustc:start=1542129836854469445,finish=1542129881272278727,duration=44417809282


[00:09:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:09:17] Build completed unsuccessfully in 0:01:47
[00:09:17] Makefile:28: recipe for target 'all' failed
[00:09:17] make: *** [all] Error 1
