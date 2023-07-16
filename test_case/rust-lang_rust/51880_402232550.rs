plain
[00:58:27] ....................................................................................................
[00:58:34] ....................................................................................................
[00:58:41] ....................................................................................................
[00:58:49] ....................................................................................................
[00:58:55] i........................................................................F.........................i
[00:59:02] ..ii................................................................................................
[00:59:16] ......................................................................test [compile-fail] compile-fail/issue-22638.rs has been running for over 60 seconds
[00:59:17] ..............................
[00:59:22] ...................................................................................i................
[00:59:28] ...............................i....................................................................
---
[00:59:45] error: /checkout/src/test/compile-fail/method-call-lifetime-args-subst-index.rs:25: expected error not found: compilation successful
[00:59:45] 
[00:59:45] error: 1 unexpected errors found, 1 expected errors not found
[00:59:45] status: exit code: 101
[00:59:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/method-call-lifetime-args-subst-index.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/method-call-lifetime-args-subst-index/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/method-call-lifetime-args-subst-index/auxiliary" "-A" "unused"
[00:59:45]     Error {
[00:59:45]         line_num: 21,
[00:59:45]         kind: Some(
[00:59:45]             Error
---
[00:59:45] 
[00:59:45] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:59:45] 
[00:59:45] 
[00:59:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:45] 
[00:59:45] 
[00:59:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:45] Build completed unsuccessfully in 0:15:11
[00:59:45] Build completed unsuccessfully in 0:15:11
[00:59:45] Makefile:58: recipe for target 'check' failed
[00:59:45] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27ffe594
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
