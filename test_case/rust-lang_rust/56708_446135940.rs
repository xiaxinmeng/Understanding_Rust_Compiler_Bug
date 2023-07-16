plain
travis_time:end:1d63d400:start=1544520950345851071,finish=1544520951421633700,duration=1075782629
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:12] expected success, got: exit code: 101
[00:04:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:12] Build completed unsuccessfully in 0:00:50
[00:04:12] Makefile:28: recipe for target 'all' failed
[00:04:12] make: *** [all] Error 1
