plain
travis_time:end:01f8f7ed:start=1552440337766441835,finish=1552440410683883821,duration=72917441986
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:11:36] .................................................................................................... 1000/2957
[01:11:49] .................................................................................................... 1100/2957
[01:11:58] .................................................................................................... 1200/2957
[01:12:08] .................................................................................................... 1300/2957
[01:12:20] ERROR 2019-03-13T02:39:20Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/run-pass/issues/auxiliary/issue_3136_a.rc` source not found"
[01:12:20] ......................F...........................................................................F. 1400/2957
[01:12:31] ..............................................................................F..................... 1500/2957
[01:12:40] ......F...............................................................i............................. 1600/2957
[01:12:53] ERROR 2019-03-13T02:39:53Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/run-pass/issues/issue24687-embed-debuginfo/auxiliary/issue24687-lib.rs` source not found"
[01:12:54] ................................................................................................F... 1700/2957
[01:13:16] .................................................................................................... 1900/2957
[01:13:30] .......i......................................................................i..................... 2000/2957
[01:13:54] .................................................................................................... 2100/2957
[01:14:17] .................................................................................................... 2200/2957
---
[01:16:07] failures:
[01:16:07] 
[01:16:07] ---- [run-pass] run-pass/issues/issue-26873-multifile.rs stdout ----
[01:16:07] 
[01:16:07] error: test compilation failed although it shouldn't!
[01:16:07] status: exit code: 1
[01:16:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-26873-multifile.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-26873-multifile/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-26873-multifile/auxiliary"
[01:16:07] ------------------------------------------
[01:16:07] 
[01:16:07] ------------------------------------------
[01:16:07] stderr:
[01:16:07] stderr:
[01:16:07] ------------------------------------------
[01:16:07] {"message":"file not found for module `issue_26873_multifile`","code":{"code":"E0583","explanation":"\nA file wasn't found for an out-of-line module.\n\nErroneous code example:\n\n