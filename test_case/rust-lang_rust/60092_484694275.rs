plain
travis_time:end:09526ea0:start=1555618297614667313,finish=1555618409389483009,duration=111774815696
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:05:56] .................................................................................................... 1200/5544
[01:05:59] .................................................................................................... 1300/5544
[01:06:01] .................................................................................................... 1400/5544
[01:06:05] .................................................................................................... 1500/5544
[01:06:07] ..........................F......................................................................... 1600/5544
[01:06:14] .................................................................................................... 1800/5544
[01:06:18] .................................................................................................... 1900/5544
[01:06:22] .................................................................................................... 2000/5544
[01:06:25] ...................................................................................................i 2100/5544
---
[01:08:40] failures:
[01:08:40] 
[01:08:40] ---- [ui] ui/feature-gates/feature-gate-async-await.rs stdout ----
[01:08:40] 
[01:08:40] error: /checkout/src/test/ui/feature-gates/feature-gate-async-await.rs:12: unexpected error: '12:5: 12:20: async fn is unstable [E0658]'
[01:08:40] error: 1 unexpected errors found, 0 expected errors not found
[01:08:40] status: exit code: 1
[01:08:40] status: exit code: 1
[01:08:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/auxiliary" "-A" "unused"
[01:08:40]     Error {
[01:08:40]         line_num: 12,
[01:08:40]         kind: Some(
[01:08:40]             Error,
[01:08:40]             Error,
[01:08:40]         ),
[01:08:40]         msg: "12:5: 12:20: async fn is unstable [E0658]",
[01:08:40] ]
[01:08:40] 
[01:08:40] thread '[ui] ui/feature-gates/feature-gate-async-await.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1400:13
[01:08:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:08:40] 
[01:08:40] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:08:40] 
[01:08:40] 
[01:08:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:40] 
[01:08:40] 
[01:08:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:40] Build completed unsuccessfully in 0:04:30
[01:08:40] Build completed unsuccessfully in 0:04:30
[01:08:40] Makefile:48: recipe for target 'check' failed
[01:08:40] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1420f81e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 18 21:22:20 UTC 2019
---
travis_time:end:36158e39:start=1555622541637450055,finish=1555622541644980615,duration=7530560
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:032327f6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0266ca6e
$ dmesg | grep -i kill
