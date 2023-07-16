plain
[00:56:34] ....................................................................................................
[00:56:35] ......................test [compile-fail] compile-fail/issue-22638.rs has been running for over 60 seconds
[00:56:39] .............................................................i................
[00:56:44] ...............................i....................................................................
[00:56:51] ....................................................F...............................................
[00:57:00] ....................................................................................................
[00:57:01] ...........................
[00:57:01] failures:
[00:57:01] 
[00:57:01] 
[00:57:01] ---- [compile-fail] compile-fail/trait-object-vs-lifetime.rs stdout ----
[00:57:01] 
[00:57:01] error: /checkout/src/test/compile-fail/trait-object-vs-lifetime.rs:26: expected error not found: at least one non-builtin trait is required for an object type
[00:57:01] error: 0 unexpected errors found, 1 expected errors not found
[00:57:01] status: exit code: 101
[00:57:01] status: exit code: 101
[00:57:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/trait-object-vs-lifetime.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/trait-object-vs-lifetime/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/trait-object-vs-lifetime/auxiliary" "-A" "unused"
[00:57:01]     Error {
[00:57:01]         line_num: 26,
[00:57:01]         kind: Some(
[00:57:01]             Error
[00:57:01]             Error
[00:57:01]         ),
[00:57:01]         msg: "at least one non-builtin trait is required for an object type"
[00:57:01] ]
[00:57:01] 
[00:57:01] thread '[compile-fail] compile-fail/trait-object-vs-lifetime.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:57:01] 
---
[00:57:01] 
[00:57:01] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:57:01] 
[00:57:01] 
[00:57:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:01] 
[00:57:01] 
[00:57:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:01] Build completed unsuccessfully in 0:14:21
[00:57:01] Build completed unsuccessfully in 0:14:21
[00:57:01] Makefile:58: recipe for target 'check' failed
[00:57:01] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18b6d69d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:11841384:start=1530647693448971804,finish=1530647693455006456,duration=6034652
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0808d4d4
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0022cd10
$ dmesg | grep -i kill
