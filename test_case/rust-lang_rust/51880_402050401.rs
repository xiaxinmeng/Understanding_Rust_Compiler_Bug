plain
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:30] 
[00:51:30] running 2428 tests
[00:51:34] ....................................................................................................
[00:51:40] ......................................F.............................................................
[00:51:50] ....................................................................................................
[00:51:56] .......................................................................................i............
[00:52:02] ................................................ii.iii..............................................
[00:52:07] ................................................................................i...................
---
[00:53:45] error: /checkout/src/test/compile-fail/bad-mid-path-type-params.rs:46: unexpected error: '46:44: 46:47: mismatched types [E0308]'
[00:53:45] 
[00:53:45] error: 2 unexpected errors found, 0 expected errors not found
[00:53:45] status: exit code: 101
[00:53:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/bad-mid-path-type-params.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/bad-mid-path-type-params/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/bad-mid-path-type-params/auxiliary" "-A" "unused"
[00:53:45] unexpected errors (from JSON output): [
[00:53:45]         line_num: 40,
[00:53:45]         kind: Some(
[00:53:45]             Error
[00:53:45]         ),
---
[00:53:45] 
[00:53:45] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:53:45] 
[00:53:45] 
[00:53:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:45] 
[00:53:45] 
[00:53:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:45] Build completed unsuccessfully in 0:13:39
[00:53:45] Build completed unsuccessfully in 0:13:39
[00:53:45] Makefile:58: recipe for target 'check' failed
[00:53:45] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05fcf1a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
