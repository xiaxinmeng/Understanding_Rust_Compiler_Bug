plain
[00:48:05] ....................................................................................................
[00:48:09] ....................................................................................................
[00:48:11] ..........i.........................................................................................
[00:48:14] ....................................................................................................
[00:48:16] ...........................................................iiiiiiiii................................
[00:48:22] ....................................................................................................
[00:48:25] ....................................................................................................
[00:48:28] .......................................i............................................................
[00:48:31] ..........................................................................................ii..ii....
---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:47] 
[00:55:47] running 29 tests
[00:56:09] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:56:09] .............F...............
[00:56:09] 
[00:56:09] ---- [ui] ui-fulldeps/lint_tool_test.rs stdout ----
[00:56:09] 
[00:56:09] 
[00:56:09] error: /checkout/src/test/ui-fulldeps/lint_tool_test.rs:17: unexpected warning: '17:9: 17:21: lint name `clippy_group` is deprecated and may not have an effect in the future [renamed_and_removed_lints]'
[00:56:09] error: 1 unexpected errors found, 0 expected errors not found
[00:56:09] status: exit code: 1
[00:56:09] status: exit code: 1
[00:56:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint_tool_test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint_tool_test/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint_tool_test/auxiliary" "-A" "unused"
[00:56:09]     Error {
[00:56:09]         line_num: 17,
[00:56:09]         kind: Some(
[00:56:09]             Warning
[00:56:09]             Warning
[00:56:09]         ),
[00:56:09]         msg: "17:9: 17:21: lint name `clippy_group` is deprecated and may not have an effect in the future [renamed_and_removed_lints]"
[00:56:09] ]
[00:56:09] 
[00:56:09] thread '[ui] ui-fulldeps/lint_tool_test.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:56:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:56:09] test result: FAILED. 28 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:56:09] 
[00:56:09] 
[00:56:09] 
[00:56:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:09] 
[00:56:09] 
[00:56:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:09] Build completed unsuccessfully in 0:11:56
[00:56:09] Build completed unsuccessfully in 0:11:56
[00:56:09] Makefile:58: recipe for target 'check' failed
[00:56:09] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00f74fad
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03728b9f:start=1535463833680044718,finish=1535463833688457486,duration=8412768
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b92b6d2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_f
