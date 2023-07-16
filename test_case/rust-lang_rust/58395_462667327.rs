plain
travis_time:end:28981f02:start=1549960318713972756,finish=1549960319584023927,duration=870051171
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:39]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:42] error[E0308]: mismatched types
[00:04:42]    --> src/libstd/time.rs:238:20
[00:04:42]     |
[00:04:42] 238 |         if self >= earlier {
[00:04:42]     |                    |
[00:04:42]     |                    expected reference, found struct `time::Instant`
[00:04:42]     |                    expected reference, found struct `time::Instant`
[00:04:42]     |                    help: consider borrowing here: `&earlier`
[00:04:42]     = note: expected type `&time::Instant`
[00:04:42]                found type `time::Instant`
[00:04:42] 
[00:04:43] error: aborting due to previous error
---
[00:04:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:43] expected success, got: exit code: 101
[00:04:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:43] Build completed unsuccessfully in 0:00:42
[00:04:43] Makefile:18: recipe for target 'all' failed
[00:04:43] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:007db223
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 08:36:53 UTC 2019
