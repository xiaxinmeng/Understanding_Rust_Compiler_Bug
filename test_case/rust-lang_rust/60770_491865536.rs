plain
travis_time:end:1c6cd95c:start=1557755714240176083,finish=1557755715210255905,duration=970079822
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:14:55] .................................................................................................... 600/2960
[01:15:10] .................................................................................................... 700/2960
[01:15:22] .................................................................................................... 800/2960
[01:15:31] .................................................................................................... 900/2960
[01:15:46] .......................................F............................................................ 1000/2960
[01:16:09] .................................................................................................... 1200/2960
[01:16:19] .................................................................................................... 1300/2960
[01:16:31] ...........................ii....................................................................... 1400/2960
[01:16:43] .................................................................................................... 1500/2960
---
[01:20:10] .................................................................................................... 2800/2960
[01:20:20] .................................................................................................... 2900/2960
g output.
[01:20:30] status: exit code: 0
[01:20:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/impl-trait-in-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait-in-bindings/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait-in-bindings/auxiliary"
[01:20:30] ------------------------------------------
[01:20:30] 
[01:20:30] ------------------------------------------
[01:20:30] stderr:
[01:20:30] stderr:
[01:20:30] ------------------------------------------
[01:20:30] warning: the feature `impl_trait_in_bindings` is incomplete and may cause the compiler to crash
[01:20:30]    |
[01:20:30] LL | #![feature(impl_trait_in_bindings)]
[01:20:30]    |            ^^^^^^^^^^^^^^^^^^^^^^
[01:20:30] 
---
[01:20:30] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:20:30] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:20:30] 
[01:20:30] 
[01:20:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:20:30] 
[01:20:30] 
[01:20:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:30] Build completed unsuccessfully in 0:11:27
[01:20:30] Build completed unsuccessfully in 0:11:27
[01:20:30] Makefile:48: recipe for target 'check' failed
[01:20:30] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e6a085e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 13 15:15:59 UTC 2019
