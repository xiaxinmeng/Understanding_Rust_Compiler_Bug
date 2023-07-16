plain
travis_time:end:09f39ace:start=1548841178820246614,finish=1548841180273601068,duration=1453354454
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:58] 
[01:08:58] running 2946 tests
[01:09:11] .................................................................................................... 100/2946
[01:09:22] ..................................................F..............................i.................. 200/2946
[01:09:43] .................................................................................................... 400/2946
[01:09:52] .................................................................................................... 500/2946
[01:10:04] .................................................................................................... 600/2946
[01:10:20] .................................................................................................... 700/2946
[01:10:20] .................................................................................................... 700/2946
[01:10:32] .................................................................................................... 800/2946
[01:10:41] ........................................F........................................................... 900/2946
[01:11:10] .................................................................................................... 1100/2946
[01:11:19] .................................................................................................... 1200/2946
[01:11:29] .................................................................................................... 1300/2946
[01:11:42] .................................................................................................... 1400/2946
---
[01:15:39] failures:
[01:15:39] 
[01:15:39] ---- [run-pass] run-pass/async-await.rs stdout ----
[01:15:39] 
[01:15:39] error: test compilation failed although it shouldn't!
[01:15:39] status: exit code: 1
[01:15:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/async-await/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/async-await/auxiliary"
[01:15:39] ------------------------------------------
[01:15:39] 
[01:15:39] ------------------------------------------
[01:15:39] stderr:
[01:15:39] stderr:
[01:15:39] ------------------------------------------
[01:15:39] {"message":"unresolved imports `std::task::LocalWaker`, `std::task::Wake`, `std::task::local_waker_from_nonlocal`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n