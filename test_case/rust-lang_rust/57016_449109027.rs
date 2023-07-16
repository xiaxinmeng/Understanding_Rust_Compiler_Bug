plain
travis_time:end:0464a560:start=1545332457594937489,finish=1545332461839140304,duration=4244202815
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:24:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:24:09] expected success, got: exit code: 101
[00:24:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:24:09] Build completed unsuccessfully in 0:20:52
[00:24:09] make: *** [all] Error 1
[00:24:09] Makefile:28: recipe for target 'all' failed
