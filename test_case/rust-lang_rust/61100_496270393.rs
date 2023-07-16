plain
travis_time:end:14b2d72a:start=1558972890026616648,finish=1558972981158521679,duration=91131905031
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:13:18] .................................................................................................... 300/2922
[01:13:29] .................................................................................................... 400/2922
[01:13:39] .................................................................................................... 500/2922
[01:13:50] .................................................................................................... 600/2922
[01:14:04] ................................F................................................................... 700/2922
[01:14:24] .................................................................................................... 900/2922
[01:14:39] .................................................................................................... 1000/2922
[01:14:50] .................................................................................................... 1100/2922
[01:14:50] .................................................................................................... 1100/2922
[01:14:59] ................................................F................................................... 1200/2922
[01:15:22] ...................ii............................................................................... 1400/2922
[01:15:34] .................................................................................................... 1500/2922
[01:15:43] .......................................................................i.......i.................... 1600/2922
[01:15:56] .................................................................................................... 1700/2922
[01:15:56] .................................................................................................... 1700/2922
[01:16:10] .........................................................F.......................................... 1800/2922
[01:16:20] ............................................................................F....................... 1900/2922
[01:16:35] ..i.......................................................................i......................... 2000/2922
[01:17:00] .................................................................................................... 2100/2922
[01:17:22] ...............................F.................................................................... 2200/2922
[01:17:33] .......................................................................................F............ 2300/2922
[01:18:02] .................................................................................................... 2500/2922
[01:18:34] .................................................................................................... 2600/2922
[01:18:44] .................................................................................................... 2700/2922
[01:18:54] .................................................................................................... 2800/2922
[01:18:54] .................................................................................................... 2800/2922
[01:19:07] .................................................................................................... 2900/2922
[01:19:11] ......................
[01:19:11] failures:
[01:19:11] 
[01:19:11] ---- [run-pass] run-pass/drop/dynamic-drop.rs stdout ----
[01:19:11] 
[01:19:11] error: test compilation failed although it shouldn't!
[01:19:11] status: exit code: 101
[01:19:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/dynamic-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop/auxiliary"
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] ------------------------------------------
[01:19:11] stderr:
---
[01:19:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:11] 
[01:19:11] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:19:11] 
[01:19:11] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:19:11] 
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] 
[01:19:11] 
[01:19:11] ---- [run-pass] run-pass/issues/issue-18110.rs stdout ----
[01:19:11] 
[01:19:11] error: test compilation failed although it shouldn't!
[01:19:11] status: exit code: 101
[01:19:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-18110.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18110/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18110/auxiliary"
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] ------------------------------------------
[01:19:11] stderr:
---
[01:19:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:11] 
[01:19:11] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:19:11] 
[01:19:11] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:19:11] 
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] 
[01:19:11] 
[01:19:11] ---- [run-pass] run-pass/macros/log_syntax-trace_macros-macro-locations.rs stdout ----
[01:19:11] 
[01:19:11] error: test compilation failed although it shouldn't!
[01:19:11] status: exit code: 101
[01:19:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/macros/log_syntax-trace_macros-macro-locations.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/log_syntax-trace_macros-macro-locations/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/log_syntax-trace_macros-macro-locations/auxiliary"
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] 
[01:19:11] 
---
[01:19:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:11] 
[01:19:11] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:19:11] 
[01:19:11] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:19:11] 
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] 
[01:19:11] 
[01:19:11] ---- [run-pass] run-pass/mir/mir_drop_order.rs stdout ----
[01:19:11] 
[01:19:11] error: test compilation failed although it shouldn't!
[01:19:11] status: exit code: 101
[01:19:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/mir/mir_drop_order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir/mir_drop_order/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir/mir_drop_order/auxiliary"
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] ------------------------------------------
[01:19:11] stderr:
---
[01:19:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:11] 
[01:19:11] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:19:11] 
[01:19:11] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:19:11] 
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] 
[01:19:11] 
[01:19:11] ---- [run-pass] run-pass/proc-macro/issue-42708.rs stdout ----
[01:19:11] 
[01:19:11] error: test compilation failed although it shouldn't!
[01:19:11] status: exit code: 101
[01:19:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/proc-macro/issue-42708.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/proc-macro/issue-42708/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/proc-macro/issue-42708/auxiliary"
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] ------------------------------------------
[01:19:11] stderr:
---
[01:19:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:11] 
[01:19:11] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:19:11] 
[01:19:11] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:19:11] 
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] 
[01:19:11] 
[01:19:11] ---- [run-pass] run-pass/rfcs/rfc1857-drop-order.rs stdout ----
[01:19:11] 
[01:19:11] error: test compilation failed although it shouldn't!
[01:19:11] status: exit code: 101
[01:19:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/rfcs/rfc1857-drop-order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfcs/rfc1857-drop-order/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfcs/rfc1857-drop-order/auxiliary"
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] ------------------------------------------
[01:19:11] stderr:
---
[01:19:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:11] 
[01:19:11] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:19:11] 
[01:19:11] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:19:11] 
[01:19:11] ------------------------------------------
[01:19:11] 
[01:19:11] 
---
[01:19:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:19:11] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:11] 
[01:19:11] 
[01:19:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:11] 
[01:19:11] 
[01:19:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:11] Build completed unsuccessfully in 0:11:15
[01:19:11] Build completed unsuccessfully in 0:11:15
[01:19:11] Makefile:48: recipe for target 'check' failed
[01:19:11] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17c34cca
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 27 17:22:21 UTC 2019
---
travis_time:end:0e0f2434:start=1558977742558166380,finish=1558977742563343976,duration=5177596
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0213ca08
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0fcaaf0d
travis_time:start:0fcaaf0d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:029e594b
$ dmesg | grep -i kill
