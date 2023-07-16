plain
travis_time:end:08f27db8:start=1556733987099447641,finish=1556734074264998063,duration=87165550422
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:13:52] .................................................................................................... 1700/2958
[01:14:05] .................................................................................................... 1800/2958
[01:14:15] .................................................................................................... 1900/2958
[01:14:29] ........i.......................................................................i................... 2000/2958
[01:14:54] .........................................................................................F.......... 2100/2958
[01:15:17] ...............................................................................................test [run-pass] run-pass/mpsc_stress.rs has been running for over 60 seconds
[01:15:28] .................................................................................................... 2300/2958
[01:15:44] ..........................................ii........................................................ 2400/2958
[01:15:58] .................................................................................................... 2500/2958
[01:16:25] .................................................................................................... 2600/2958
---
[01:17:10] normalized stderr:
[01:17:10] warning: the feature `maybe_uninit` has been stable since 1.36.0 and no longer requires an attribute to enable
[01:17:10]   --> $DIR/panic-uninitialized-zeroed.rs:5:24
[01:17:10]    |
[01:17:10] LL | #![feature(never_type, maybe_uninit)]
[01:17:10]    |
[01:17:10]    = note: #[warn(stable_features)] on by default
[01:17:10] 
[01:17:10] 
[01:17:10] 
[01:17:10] 
[01:17:10] 
[01:17:10] The actual stderr differed from the expected stderr.
[01:17:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-uninitialized-zeroed/panic-uninitialized-zeroed.stderr
[01:17:10] To update references, rerun the tests and pass the `--bless` flag
[01:17:10] To only update this specific test, also pass `--test-args panic-uninitialized-zeroed.rs`
[01:17:10] error: 1 errors occurred comparing output.
[01:17:10] status: exit code: 0
[01:17:10] status: exit code: 0
[01:17:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/panic-uninitialized-zeroed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-uninitialized-zeroed/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-uninitialized-zeroed/auxiliary"
[01:17:10] ------------------------------------------
[01:17:10] 
[01:17:10] ------------------------------------------
[01:17:10] stderr:
[01:17:10] stderr:
[01:17:10] ------------------------------------------
[01:17:10] warning: the feature `maybe_uninit` has been stable since 1.36.0 and no longer requires an attribute to enable
[01:17:10]   --> /checkout/src/test/run-pass/panic-uninitialized-zeroed.rs:5:24
[01:17:10]    |
[01:17:10] LL | #![feature(never_type, maybe_uninit)]
[01:17:10]    |
[01:17:10]    = note: #[warn(stable_features)] on by default
[01:17:10] 
[01:17:10] 
---
[01:17:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:17:10] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:17:10] 
[01:17:10] 
[01:17:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:10] 
[01:17:10] 
[01:17:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:10] Build completed unsuccessfully in 0:10:45
[01:17:10] Build completed unsuccessfully in 0:10:45
[01:17:10] make: *** [check] Error 1
[01:17:10] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2e44e00e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  1 19:25:13 UTC 2019
