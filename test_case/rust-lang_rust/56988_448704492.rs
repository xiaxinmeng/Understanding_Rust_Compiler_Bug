plain
travis_time:end:15d5c828:start=1545245110625471104,finish=1545245113003082919,duration=2377611815
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:03]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:08] error: unused import: `Duration`
[00:06:08]   --> src/librustc/util/profiling.rs:15:17
[00:06:08]    |
[00:06:08] 15 | use std::time::{Duration, Instant};
[00:06:08]    |
[00:06:08]    = note: `-D unused-imports` implied by `-D warnings`
[00:06:08] 
[00:06:34] error: aborting due to previous error
---
[00:06:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:34] expected success, got: exit code: 101
[00:06:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:34] Build completed unsuccessfully in 0:03:46
[00:06:34] make: *** [all] Error 1
[00:06:34] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0af64a28
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 19 18:51:56 UTC 2018
