plain
travis_time:end:02736bfb:start=1559415623244391238,finish=1559415712430619061,duration=89186227823
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:00:33] .................................................................................................... 1500/2923
[01:00:42] .......................................................................i........i................... 1600/2923
[01:00:55] .................................................................................................... 1700/2923
[01:01:09] .................................................................................................... 1800/2923
[01:01:19] ......................................................F............................................. 1900/2923
[01:01:33] ..i.......................................................................i......................... 2000/2923
[01:02:20] .................................................................................................... 2200/2923
[01:02:30] .................................................................................................... 2300/2923
[01:02:49] ........ii.......................................................................................... 2400/2923
[01:02:59] .................................................................................................... 2500/2923
---
[01:04:06] failures:
[01:04:06] 
[01:04:06] ---- [run-pass] run-pass/mir/mir-inlining/no-trait-method-issue-40473.rs stdout ----
[01:04:06] 
[01:04:06] error: test compilation failed although it shouldn't!
[01:04:06] status: exit code: 101
[01:04:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/mir/mir-inlining/no-trait-method-issue-40473.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir/mir-inlining/no-trait-method-issue-40473/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir/mir-inlining/no-trait-method-issue-40473/auxiliary"
[01:04:06] ------------------------------------------
[01:04:06] 
[01:04:06] ------------------------------------------
[01:04:06] stderr:
---
[01:04:06] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:06] 
[01:04:06] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:04:06] 
[01:04:06] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z mir-opt-level=2 -C prefer-dynamic -C rpath -C debuginfo=0
[01:04:06] 
[01:04:06] ------------------------------------------
[01:04:06] 
[01:04:06] 
---
[01:04:06] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:04:06] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:06] 
[01:04:06] 
[01:04:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:06] 
[01:04:06] 
[01:04:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:06] Build completed unsuccessfully in 1:00:14
