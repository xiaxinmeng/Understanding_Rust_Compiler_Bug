plain
travis_time:end:0c29258a:start=1558719374888267524,finish=1558719462301571650,duration=87413304126
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:11:59] .................................................................................................... 2700/5579
[01:12:04] .................................................................................................... 2800/5579
[01:12:07] .................................................................................................... 2900/5579
[01:12:11] .................................................................................................... 3000/5579
[01:12:14] .....................F.............................................................................. 3100/5579
[01:12:22] .................................................................................................... 3300/5579
[01:12:25] ...........i........................................................................................ 3400/5579
[01:12:28] .....................................................................................ii...i..ii..... 3500/5579
[01:12:32] .................................................................................................... 3600/5579
---
[01:13:48] failures:
[01:13:48] 
[01:13:48] ---- [ui] ui/issues/issue-6991.rs stdout ----
[01:13:48] 
[01:13:48] error: test compilation failed although it shouldn't!
[01:13:48] status: exit code: 101
[01:13:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-6991.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6991" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6991/auxiliary" "-A" "unused"
[01:13:48] ------------------------------------------
[01:13:48] 
[01:13:48] ------------------------------------------
[01:13:48] stderr:
[01:13:48] stderr:
[01:13:48] ------------------------------------------
[01:13:48] error: internal compiler error: src/librustc_mir/interpret/place.rs:604: eval_place_to_mplace called on (*(x: &usize))
[01:13:48] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
[01:13:48] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:48] error: aborting due to previous error
[01:13:48] 
[01:13:48] 
[01:13:48] 
[01:13:48] note: the compiler unexpectedly panicked. this is a bug.
[01:13:48] 
[01:13:48] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:13:48] 
[01:13:48] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:13:48] 
[01:13:48] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:13:48] 
[01:13:48] ------------------------------------------
[01:13:48] 
[01:13:48] 
---
[01:13:48] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:13:48] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:48] 
[01:13:48] 
[01:13:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:48] 
[01:13:48] 
[01:13:48] 
[01:13:48] make: *** [check] Error 1
[01:13:48] Build completed unsuccessfully in 0:04:35
[01:13:48] Makefile:48: recipe for target 'check' failed
travis_time:end:3586747a:start=1558719471535903157,finish=1558723900183850082,duration=4428647946925
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
