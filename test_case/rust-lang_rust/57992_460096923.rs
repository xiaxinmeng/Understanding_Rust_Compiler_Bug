plain
travis_time:end:01837f16:start=1549230507215870171,finish=1549230509419569502,duration=2203699331
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:00:56] .................................................................................................... 500/2946
[01:01:07] .................................................................................................... 600/2946
[01:01:22] .................................................................................................... 700/2946
[01:01:33] .................................................................................................... 800/2946
[01:01:42] ........................................F........................................................... 900/2946
[01:02:11] .................................................................................................... 1100/2946
[01:02:20] .................................................................................................... 1200/2946
[01:02:30] .................................................................................................... 1300/2946
[01:02:42] .................................................................................................... 1400/2946
---
[01:06:32] failures:
[01:06:32] 
[01:06:32] ---- [run-pass] run-pass/futures-api.rs stdout ----
[01:06:32] 
[01:06:32] error: test compilation failed although it shouldn't!
[01:06:32] status: exit code: 1
[01:06:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/futures-api.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/futures-api/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/futures-api/auxiliary"
[01:06:32] ------------------------------------------
[01:06:32] 
[01:06:32] ------------------------------------------
[01:06:32] stderr:
[01:06:32] stderr:
[01:06:32] ------------------------------------------
[01:06:32] {"message":"cannot find trait `Wake` in this scope","code":{"code":"E0405","explanation":"\nThe code refers to a trait that is not in scope.\n\nErroneous code example:\n\n