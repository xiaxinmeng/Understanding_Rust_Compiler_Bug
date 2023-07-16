plain
[00:49:01] ....................................................................................................
[00:49:05] ....................................................................................................
[00:49:07] ..........i.........................................................................................
[00:49:10] ....................................................................................................
[00:49:13] ...........................................................iiiiiiiii................................
[00:49:18] ....................................................................................................
[00:49:22] ....................................................................................................
[00:49:24] .......................................i............................................................
[00:49:27] ..........................................................................................ii...ii...
---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:49] 
[00:56:49] running 29 tests
[00:57:11] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:57:11] ............F................
[00:57:11] 
[00:57:11] ---- [ui] ui-fulldeps/lint_tool_test.rs stdout ----
[00:57:11] 
[00:57:11] 
[00:57:11] error: /checkout/src/test/ui-fulldeps/lint_tool_test.rs:17: unexpected warning: '17:9: 17:21: lint name `clippy_group` is deprecated and may not have an effect in the future [renamed_and_removed_lints]'
[00:57:11] error: 1 unexpected errors found, 0 expected errors not found
[00:57:11] status: exit code: 1
[00:57:11] status: exit code: 1
[00:57:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint_tool_test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint_tool_test/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint_tool_test/auxiliary" "-A" "unused"
[00:57:11]     Error {
[00:57:11]         line_num: 17,
[00:57:11]         kind: Some(
[00:57:11]             Warning
[00:57:11]             Warning
[00:57:11]         ),
[00:57:11]         msg: "17:9: 17:21: lint name `clippy_group` is deprecated and may not have an effect in the future [renamed_and_removed_lints]"
[00:57:11] ]
[00:57:11] 
[00:57:11] thread '[ui] ui-fulldeps/lint_tool_test.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:57:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:57:11] test result: FAILED. 28 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:57:11] 
[00:57:11] 
[00:57:11] 
[00:57:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:11] 
[00:57:11] 
[00:57:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:11] Build completed unsuccessfully in 0:12:12
[00:57:11] Build completed unsuccessfully in 0:12:12
[00:57:11] make: *** [check] Error 1
[00:57:11] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b27faee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:026da824:start=1535570453026000231,finish=1535570453033880253,duration=7880022
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02bec9a8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02009620
travis_time:start:02009620
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11faae0c
$ dmesg | grep -i kill
