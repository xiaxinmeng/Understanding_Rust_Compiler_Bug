plain
travis_time:end:1bb62a10:start=1549072615354026354,finish=1549072690243356171,duration=74889329817
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:34] .................................................................................................... 100/2946
[00:59:46] .................................................................................i.................. 200/2946
[00:59:54] .................................................................................................... 300/2946
[01:00:05] .................................................................................................... 400/2946
[01:00:14] ..........................................F..F...................................................... 500/2946
[01:00:40] .................................................................................................... 700/2946
[01:00:51] .................................................................................................... 800/2946
[01:00:59] .................................................................................................... 900/2946
[01:01:15] .................................................................................................... 1000/2946
---
[01:05:44] failures:
[01:05:44] 
[01:05:44] ---- [run-pass] run-pass/const-int-overflowing.rs stdout ----
[01:05:44] 
[01:05:44] error: test compilation failed although it shouldn't!
[01:05:44] status: exit code: 1
[01:05:44] status: exit code: 1
[01:05:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/const-int-overflowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-int-overflowing/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-int-overflowing/auxiliary"
[01:05:44] ------------------------------------------
[01:05:44] 
[01:05:44] ------------------------------------------
[01:05:44] stderr:
[01:05:44] stderr:
[01:05:44] ------------------------------------------
[01:05:44] {"message":"no method named `overflowing_neg` found for type `{integer}` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n