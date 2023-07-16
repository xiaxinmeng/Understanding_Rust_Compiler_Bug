plain
travis_time:end:1e189384:start=1560362267693801950,finish=1560362363448156592,duration=95754354642
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:01:51] .................................................................................................... 2100/2920
[01:02:13] .................................................................................................... 2200/2920
[01:02:14] .............test [run-pass] run-pass/mpsc_stress.rs has been running for over 60 seconds
[01:02:23] ....................................................................................... 2300/2920
[01:02:41] ........ii..........................................................F............................... 2400/2920
[01:03:23] .................................................................................................... 2600/2920
[01:03:32] .................................................................................................... 2700/2920
[01:03:42] .................................................................................................... 2800/2920
[01:03:54] .................................................................................................... 2900/2920
[01:03:54] .................................................................................................... 2900/2920
[01:03:57] ....................
[01:03:57] failures:
[01:03:57] 
[01:03:57] ---- [run-pass] run-pass/statics/static-recursive.rs stdout ----
[01:03:57] 
[01:03:57] error: test compilation failed although it shouldn't!
[01:03:57] status: exit code: 101
[01:03:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/statics/static-recursive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/statics/static-recursive/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/statics/static-recursive/auxiliary"
[01:03:57] ------------------------------------------
[01:03:57] 
[01:03:57] ------------------------------------------
[01:03:57] stderr:
---
[01:03:57] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:03:57] 
[01:03:57] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:03:57] 
[01:03:57] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:03:57] 
[01:03:57] ------------------------------------------
[01:03:57] 
[01:03:57] 
---
[01:03:57] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:03:57] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:03:57] 
[01:03:57] 
[01:03:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:57] 
[01:03:57] 
[01:03:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:57] Build completed unsuccessfully in 0:59:51
---
travis_time:end:1446a883:start=1560366212864650574,finish=1560366212869498993,duration=4848419
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:004e4467
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03ecbb40
travis_time:start:03ecbb40
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07bcd09b
$ dmesg | grep -i kill
