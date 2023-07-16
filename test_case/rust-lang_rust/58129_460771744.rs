plain
travis_time:end:00958d00:start=1549390604095586923,finish=1549390678935354178,duration=74839767255
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:08:51] .................................................................................................... 1700/2946
[01:09:04] .................................................................................................... 1800/2946
[01:09:14] ..................................................................................................i. 1900/2946
[01:09:29] .....................................................................i.............................. 2000/2946
[01:09:54] ..............................................................................F..................... 2100/2946
[01:10:16] .......................................................................................test [run-pass] run-pass/mpsc_stress.rs has been running for over 60 seconds
[01:10:28] ...................................................................ii............................... 2300/2946
[01:10:45] .....................................i.............................................................. 2400/2946
[01:10:59] .................................................................................................... 2500/2946
[01:11:31] .................................................................................................... 2600/2946
---
[01:12:11] failures:
[01:12:11] 
[01:12:11] ---- [run-pass] run-pass/panic-uninitialized-zeroed.rs stdout ----
[01:12:11] 
[01:12:11] error: test compilation failed although it shouldn't!
[01:12:11] status: exit code: 1
[01:12:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/panic-uninitialized-zeroed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-uninitialized-zeroed/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-uninitialized-zeroed/auxiliary"
[01:12:11] ------------------------------------------
[01:12:11] 
[01:12:11] ------------------------------------------
[01:12:11] stderr:
[01:12:11] stderr:
[01:12:11] ------------------------------------------
[01:12:11] {"message":"method `into_inner` is private","code":{"code":"E0624","explanation":"\nA private item was used outside of its scope.\n\nErroneous code example:\n\n