plain
travis_time:end:12f5b80b:start=1557102582388004760,finish=1557102668679689731,duration=86291684971
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:13:54] .................................................................................................... 500/2959
[01:14:06] .................................................................................................... 600/2959
[01:14:21] .................................................................................................... 700/2959
[01:14:32] .................................................................................................... 800/2959
[01:14:41] ....................................................F............................................... 900/2959
[01:15:09] .................................................................................................... 1100/2959
[01:15:18] .................................................................................................... 1200/2959
[01:15:28] .................................................................................................... 1300/2959
[01:15:41] ..........................ii........................................................................ 1400/2959
---
[01:19:34] failures:
[01:19:34] 
[01:19:34] ---- [run-pass] run-pass/generator/issue-59972.rs stdout ----
[01:19:34] 
[01:19:34] error: test compilation failed although it shouldn't!
[01:19:34] status: exit code: 1
[01:19:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/generator/issue-59972.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/issue-59972/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/issue-59972/auxiliary"
[01:19:34] ------------------------------------------
[01:19:34] 
[01:19:34] ------------------------------------------
[01:19:34] stderr:
[01:19:34] stderr:
[01:19:34] ------------------------------------------
[01:19:34] error[E0670]: `async fn` is not permitted in the 2015 edition
[01:19:34]    |
[01:19:34]    |
[01:19:34] LL | async fn noop() { }
[01:19:34] 
[01:19:34] 
[01:19:34] error[E0670]: `async fn` is not permitted in the 2015 edition
[01:19:34]    |
[01:19:34]    |
[01:19:34] LL | async fn contains_never() {
[01:19:34] 
[01:19:34] error: aborting due to 2 previous errors
[01:19:34] 
[01:19:34] For more information about this error, try `rustc --explain E0670`.
---
[01:19:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:19:34] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:34] 
[01:19:34] 
[01:19:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:34] 
[01:19:34] 
[01:19:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:34] Build completed unsuccessfully in 0:11:05
[01:19:34] Build completed unsuccessfully in 0:11:05
[01:19:34] Makefile:48: recipe for target 'check' failed
[01:19:34] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c71ce94
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May  6 01:50:52 UTC 2019
