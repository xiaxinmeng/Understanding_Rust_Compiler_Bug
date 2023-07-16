plain
travis_time:end:00fa4436:start=1542390003290219224,finish=1542390006912398177,duration=3622178953
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:40] .................................................................................................... 100/5021
[00:50:43] .................................................................................................... 200/5021
[00:50:46] .............................ii............................................ii...................ii.. 300/5021
[00:50:49] ..............................................................................................iii... 400/5021
[00:50:52] .....iiiiiiii.iii............................iii...........................................i........ 500/5021
[00:50:59] .................................................................................................... 700/5021
[00:51:05] .................................................................................i...........i...... 800/5021
[00:51:08] .................................................................................................... 900/5021
[00:51:12] iiiii..................ii.iiii...................................................................... 1000/5021
---
[00:51:48] .................................................................................................... 2200/5021
[00:51:52] .................................................................................................... 2300/5021
[00:51:56] .................................................................................................... 2400/5021
[00:51:59] .................................................................................................... 2500/5021
[00:52:03] .................................................................................iiiiiiiii.......... 2600/5021
[00:52:10] ...............................................ii................................................... 2800/5021
[00:52:13] .................................................................................................... 2900/5021
[00:52:17] .................................................................................................... 3000/5021
[00:52:20] ..........................................i......................................................... 3100/5021
---
travis_time:start:test_compile-fail
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:29] 
[01:05:29] running 22 tests
[01:05:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:05:30] .....F....F...........
[01:05:30] 
[01:05:30] ---- [compile-fail] compile-fail/const-fn-error.rs stdout ----
[01:05:30] 
[01:05:30] 
[01:05:30] error: /checkout/src/test/compile-fail/const-fn-error.rs:19: unexpected error: '19:14: 19:18: evaluation of constant value failed [E0080]'
[01:05:30] error: 1 unexpected errors found, 0 expected errors not found
[01:05:30] status: exit code: 1
[01:05:30] status: exit code: 1
[01:05:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-fn-error.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-error/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-error/auxiliary" "-A" "unused"
[01:05:30]     Error {
[01:05:30]         line_num: 19,
[01:05:30]         kind: Some(
[01:05:30]             Error
[01:05:30]             Error
[01:05:30]         ),
[01:05:30]         msg: "19:14: 19:18: evaluation of constant value failed [E0080]"
[01:05:30] ]
[01:05:30] 
[01:05:30] thread '[compile-fail] compile-fail/const-fn-error.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1358:13
[01:05:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:30] 
[01:05:30] ---- [compile-fail] compile-fail/issue-52443.rs stdout ----
[01:05:30] 
[01:05:30] error: /checkout/src/test/compile-fail/issue-52443.rs:15: unexpected error: '15:21: 15:29: evaluation of constant value failed [E0080]'
[01:05:30] error: 1 unexpected errors found, 0 expected errors not found
[01:05:30] status: exit code: 1
[01:05:30] status: exit code: 1
[01:05:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-52443.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-52443/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-52443/auxiliary" "-A" "unused"
[01:05:30]     Error {
[01:05:30]         line_num: 15,
[01:05:30]         kind: Some(
[01:05:30]             Error
[01:05:30]             Error
[01:05:30]         ),
[01:05:30]         msg: "15:21: 15:29: evaluation of constant value failed [E0080]"
[01:05:30] ]
[01:05:30] 
[01:05:30] thread '[compile-fail] compile-fail/issue-52443.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1358:13
[01:05:30] 
---
[01:05:30] test result: FAILED. 20 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:05:30] 
[01:05:30] 
[01:05:30] 
[01:05:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:30] 
[01:05:30] 
[01:05:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:30] Build completed unsuccessfully in 0:18:37
[01:05:30] Build completed unsuccessfully in 0:18:37
[01:05:30] Makefile:58: recipe for target 'check' failed
[01:05:30] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18acb423
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 16 18:45:46 UTC 2018
---
travis_time:end:0f6c1a40:start=1542393947970375183,finish=1542393947975289437,duration=4914254
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2fe0b000
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:28b719c2
travis_time:start:28b719c2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16e63fff
$ dmesg | grep -i kill
