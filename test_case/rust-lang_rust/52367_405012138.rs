plain
[00:51:14] i...................................................................................................
[00:51:18] ....................................................................................................
[00:51:21] ....................................................................................................
[00:51:25] ....................................................................................................
[00:51:29] ......F.............................................................................................
[00:51:36] .....i..............................................................................................
[00:51:38] ....i..ii...........................................................................................
[00:51:42] ....................................................................................................
[00:51:45] ....................................................................................................
---
[00:52:01] error: /checkout/src/test/compile-fail/issue-43988.rs:34: expected error not found: attribute should not be applied to a statement
[00:52:01] 
[00:52:01] error: 0 unexpected errors found, 2 expected errors not found
[00:52:01] status: exit code: 101
[00:52:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-43988.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-43988/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-43988/auxiliary" "-A" "unused"
[00:52:01]     Error {
[00:52:01]         line_num: 24,
[00:52:01]         kind: Some(
[00:52:01]         kind: Some(
[00:52:01]    -5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:01] 
[00:52:01] 
[00:52:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:01] Build completed unsuccessfully in 0:09:20
[00:52:01] Build completed unsuccessfully in 0:09:20
[00:52:01] make: *** [check] Error 1
[00:52:01] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06c61818
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
