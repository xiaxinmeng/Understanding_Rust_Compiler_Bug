plain
travis_time:end:0ddcaee3:start=1555616568253926015,finish=1555616675891011285,duration=107637085270
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:03:52] .................................................................................................... 4400/5544
[01:03:56] .................................................................................................... 4500/5544
[01:03:59] .................................................................................................... 4600/5544
[01:04:02] .................................................................................................... 4700/5544
[01:04:08] ................F................................................................................... 4800/5544
[01:04:14] .................................................................................................... 5000/5544
[01:04:18] .................................................................................................... 5100/5544
[01:04:22] .................................................................................................... 5200/5544
[01:04:25] .................................................................................................... 5300/5544
[01:04:25] .................................................................................................... 5300/5544
[01:04:28] .................................................................................................... 5400/5544
[01:04:30] ..................................................................................i................. 5500/5544
[01:04:32] ............................................
[01:04:32] failures:
[01:04:32] 
[01:04:32] ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
[01:04:32] 
[01:04:32] error: test compilation failed although it shouldn't!
[01:04:32] status: exit code: 1
[01:04:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/auxiliary" "-A" "unused"
[01:04:32] ------------------------------------------
[01:04:32] 
[01:04:32] ------------------------------------------
[01:04:32] stderr:
[01:04:32] stderr:
[01:04:32] ------------------------------------------
[01:04:32] error[E0308]: mismatched types
[01:04:32]   --> /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:59:5
[01:04:32]    |
[01:04:32] LL |     assert_eq!(("Yeah",), dbg!("Yeah", ));
[01:04:32]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected tuple, found &str
[01:04:32]    |
[01:04:32]    = note: expected type `(&str,)`
[01:04:32]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:04:32] 
[01:04:32] error: aborting due to previous error
[01:04:32] 
---
[01:04:32] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:04:32] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:32] 
[01:04:32] 
[01:04:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:32] 
[01:04:32] 
[01:04:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:32] Build completed unsuccessfully in 0:04:10
[01:04:32] Build completed unsuccessfully in 0:04:10
[01:04:32] Makefile:48: recipe for target 'check' failed
[01:04:32] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2c7c7910
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 18 20:49:16 UTC 2019
---
travis_time:end:0bab5f40:start=1555620557990595354,finish=1555620557997139523,duration=6544169
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a444510
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d81b3fb
$ dmesg | grep -i kill
