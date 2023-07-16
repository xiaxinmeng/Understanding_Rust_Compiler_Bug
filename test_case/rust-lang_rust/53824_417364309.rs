plain
[00:48:07] ....................................................................................................
[00:48:10] ....................................................................................................
[00:48:13] ...........i........................................................................................
[00:48:16] ....................................................................................................
[00:48:18] ............................................................iiiiiiiii...............................
[00:48:24] ....................................................................................................
[00:48:28] ....................................................................................................
[00:48:30] ........................................i...........................................................
[00:48:33] ..........................................................................................i.i..ii...
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:21] 
[00:56:21] running 96 tests
[00:58:08] .................F...............................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[00:59:32] failures:
[00:59:32] 
[00:59:32] ---- [run-pass] run-pass-fulldeps/issue-16723.rs stdout ----
[00:59:32] 
[00:59:32] 
[00:59:32] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-16723.rs" failed to compile: 
[00:59:32] status: exit code: 1
[00:59:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-16723.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-16723/auxiliary" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-16723/auxiliary"
[00:59:32] ------------------------------------------
[00:59:32] 
[00:59:32] ------------------------------------------
[00:59:32] stderr:
[00:59:32] stderr:
[00:59:32] ------------------------------------------
[00:59:32] error: incorrect close delimiter: `)`
[00:59:32]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-16723.rs:37:7
[00:59:32]    |
[00:59:32] 33 |           -> Box<MacResult+'static> {
[00:59:32]    |                                     - unclosed delimiter
[00:59:32] 37 |     ]))
[00:59:32]    |       ^ incorrect close delimiter
[00:59:32] 
[00:59:32] 
[00:59:32] error: unexpected close delimiter: `}`
[00:59:32]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-16723.rs:38:1
[00:59:32] 38 | }
[00:59:32] 38 | }
[00:59:32]    | ^ unexpected close delimiter
[00:59:32] error: aborting due to 2 previous errors
[00:59:32] 
[00:59:32] 
[00:59:32] ------------------------------------------
---
[00:59:32] 
[00:59:32] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:59:32] 
[00:59:32] 
[00:59:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:32] 
[00:59:32] 
[00:59:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:32] Build completed unsuccessfully in 0:15:29
[00:59:32] Build completed unsuccessfully in 0:15:29
[00:59:32] make: *** [check] Error 1
[00:59:32] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00e872cb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:06038492:start=1535643405697723459,finish=1535643405817954976,duration=120231517
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:062df2a6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05d8cfd4
$ dmesg | grep -i kill
