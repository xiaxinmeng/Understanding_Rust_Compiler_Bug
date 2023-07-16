plain
[00:51:29] ....................................................................................................
[00:51:32] ....................................................................................................
[00:51:34] ....................................................................................................
[00:51:37] ....................................................................................................
[00:51:41] ....................................................FFF...............F.F..................i........
[00:51:46] ....................................................................................i...............
[00:51:49] .............................i......................................................................
[00:51:49] .............................i......................................................................
[00:51:53] ....................................F...............................................................
[00:52:00] .......................................................i..ii........................................
[00:52:04] ....................................................................................................
[00:52:08] ....................................................................................................
[00:52:10] ....................................................................................................
[00:52:10] ....................................................................................................
[00:52:13] .......................................i............................................................
[00:52:16] ....................................................................................................
[00:52:19] ....................................................................................................
let bindings in constants are unstable (see issue #48821) [E0658]'
[00:52:23] 
[00:52:23] error: /checkout/src/test/compile-fail/const-block-non-item-statement.rs:12: unexpected error: '12:11: 12:27: statements in constants are unstable (see issue #48821) [E0658]'
[00:52:23] error: 2 unexpected errors found, 0 expected errors not found
[00:52:23] status: exit code: 1
[00:52:23] status: exit code: 1
[00:52:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-block-non-item-statement.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-block-non-item-statement/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-block-non-item-statement/auxiliary" "-A" "unused"
[00:52:23]     Error {
[00:52:23]         line_num: 12,
[00:52:23]         kind: Some(
[00:52:23]             Error
[00:52:23]             Error
[00:52:23]         ),
[00:52:23]         msg: "12:11: 12:27: let bindings in constants are unstable (see issue #48821) [E0658]"
[00:52:23]     Error {
[00:52:23]         line_num: 12,
[00:52:23]         kind: Some(
[00:52:23]             Error
[00:52:23]             Error
[00:52:23]         ),
[00:52:23]         msg: "12:11: 12:27: statements in constants are unstable (see issue #48821) [E0658]"
[00:52:23] ]
[00:52:23] 
[00:52:23] 
[00:52:23] thread '[compile-fail] compile-fail/const-blo/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-not-safe-for-const/auxiliary" "-A" "unused"
[00:52:23]     Error {
[00:52:23]         line_num: 35,
[00:52:23]         kind: Some(
[00:52:23]             Error
[00:52:23]             Error
[00:52:23]         ),
[00:52:23]         msg: "35:5: 35:7: constant functions cannot refer to statics, use a constant instead [E0013]"
[00:52:23] ]
[00:52:23] 
[00:52:23] thread '[compile-fail] compile-fail/const-fn-not-safe-for-const.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1281:13
[00:52:23] 
---
[00:52:23] 
[00:52:23] error: /checkout/src/test/compile-fail/issue32829.rs:84: expected error not found: statements in statics are unstable
[00:52:23] 
[00:52:23] error: 0 unexpected errors found, 6 expected errors not found
[00:52:23] build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:23] 
[00:52:23] 
[00:52:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:23] Build completed unsuccessfully in 0:09:02
[00:52:23] Build completed unsuccessfully in 0:09:02
[00:52:23] make: *** [check] Error 1
[00:52:23] Makefile:58: recipe for target 'check' failed
