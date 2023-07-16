plain
travis_time:end:106f9710:start=1550082706667581869,finish=1550082783207166205,duration=76539584336
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:12:03] .................................................................................................... 2100/2949
[01:12:21] ........................................................test [run-pass] run-pass/mpsc_stress.rs has been running for over 60 seconds
[01:12:26] ............................................ 2200/2949
[01:12:37] .....................................................................ii............................. 2300/2949
[01:12:54] .......................................i.........................................................F.. 2400/2949
[01:13:40] .................................................................................................... 2600/2949
[01:13:52] .................................................................................................... 2700/2949
[01:14:03] .................................................................................................... 2800/2949
[01:14:15] .................................................................................................... 2900/2949
[01:14:15] .................................................................................................... 2900/2949
[01:14:22] .................................................
[01:14:22] failures:
[01:14:22] 
[01:14:22] ---- [run-pass] run-pass/statics/static-recursive.rs stdout ----
[01:14:22] 
[01:14:22] error: test compilation failed although it shouldn't!
[01:14:22] status: exit code: 1
[01:14:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/statics/static-recursive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/statics/static-recursive/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/statics/static-recursive/auxiliary"
[01:14:22] ------------------------------------------
[01:14:22] 
[01:14:22] ------------------------------------------
[01:14:22] stderr:
[01:14:22] stderr:
[01:14:22] ------------------------------------------
[01:14:22] {"message":"cycle detected when const-evaluating `L1`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n