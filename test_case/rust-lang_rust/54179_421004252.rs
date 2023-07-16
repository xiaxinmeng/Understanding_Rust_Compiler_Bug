plain
[00:45:08] ....................................................................................................
[00:45:10] ....................................................................................................
[00:45:12] ....................................................................................................
[00:45:15] ...............................i..................................................................i.
[00:45:18] ..................................F.F...............................................................
[00:45:24] ......................................................................................i.............
[00:45:26] ....................................................................................................
[00:45:29] ....................................................................................................
[00:45:32] ....................................................................................................
---
[00:50:46] .............................................................i......................................
[00:50:49] ....................................................................................................
[00:50:52] ....................................................................................................
[00:50:55] ....................................................................................................
at" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-with-and/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-with-and/auxiliary" "-A" "unused"
[00:50:57]     Error {
[00:50:57]         line_num: 14,
[00:50:57]         kind: Some(
[00:50:57]             Error
[00:50:57]             Error
[00:50:57]         ),
[00:50:57]         msg: "14:15: 14:18: expected `{`, found `and`"
[00:50:57] ]
[00:50:57] 
[00:50:57] not found errors (from test file): [
[00:50:57]     Error {
[00:50:57]     Error {
[00:50:57]         line_num: 16,
[00:50:57]         kind: Some(
[00:50:57]             Error
[00:50:57]         ),
[00:50:57]         msg: "expected `{`, found `}`"
[00:50:57] ]
[00:50:57] 
[00:50:57] thread '[ui] ui/if/if-with-and.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:50:57] 
---
[00:50:57] 
[00:50:57] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[00:50:57] 
[00:50:57] 
[00:50:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:57] 
[00:50:57] 
[00:50:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:57] Build completed unsuccessfully in 0:07:27
[00:50:57] Build completed unsuccessfully in 0:07:27
[00:50:57] Makefile:58: recipe for target 'check' failed
[00:50:57] make: *** [check] Error 1
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14d03fc2
$ dmesg | grep -i kill
