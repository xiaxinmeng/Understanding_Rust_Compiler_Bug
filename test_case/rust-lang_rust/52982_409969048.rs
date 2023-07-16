plain
[00:46:58] ....................................................................................................
[00:47:01] ....................................................................................................
[00:47:04] ...........................................................................................i........
[00:47:07] ...................................................ii.iii...........................................
[00:47:10] ........................................................................F...........i...............
[00:47:16] ....................................................................................................
[00:47:19] .............................................................i......................................
[00:47:21] ........................................................i..ii.......................................
[00:47:24] ....................................................................................................
[00:47:24] ....................................................................................................
[00:47:27] ....................................................................................................
[00:47:29] ....................................................................................................
[00:47:32] ........................................i...........................................................
[00:47:35] ....................................................................................................
[00:47:37] ....................................................................................................
[00:47:39] ..............................F.....................................................................
[00:47:42] failures:
[00:47:42] 
[00:47:42] ---- [compile-fail] compile-fail/feature-gate-tool_attributes.rs stdout ----
[00:47:42] 
[00:47:42] 
[00:47:42] error: compile-fail test compiled successfully!
[00:47:42] status: exit code: 0
[00:47:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/feature-gate-tool_attributes.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate-tool_attributes/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate-tool_attributes/auxiliary" "-A" "unused"
[00:47:42] ------------------------------------------
[00:47:42] 
[00:47:42] ------------------------------------------
[00:47:42] stderr:
---
[00:47:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:42] 
[00:47:42] ---- [compile-fail] compile-fail/unknown_tool_attributes-1.rs stdout ----
[00:47:42] 
[00:47:42] error: /checkout/src/test/compile-fail/unknown_tool_attributes-1.rs:15: expected error not found: scoped attribute `foo::bar` is experimental (see issue #44690) [E0658]
[00:47:42] error: 0 unexpected errors found, 1 expected errors not found
[00:47:42] status: exit code: 1
[00:47:42] status: exit code: 1
[00:47:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/unknown_tool_attributes-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/unknown_tool_attributes-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/unknown_tool_attributes-1/auxiliary" "-A" "unused"
[00:47:42]     Error {
[00:47:42]         line_num: 15,
[00:47:42]         kind: Some(
[00:47:42]             Error
[00:47:42]             Error
[00:47:42]         ),
[00:47:42]         msg: "scoped attribute `foo::bar` is experimental (see issue #44690) [E0658]"
[00:47:42] ]
[00:47:42] 
[00:47:42] thread '[compile-fail] compile-fail/unknown_tool_attributes-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1285:13
[00:47:42] 
---
[00:47:42] 
[00:47:42] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:47:42] 
[00:47:42] 
[00:47:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:42] 
[00:47:42] 
[00:47:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:42] Build completed unsuccessfully in 0:08:36
[00:47:42] Build completed unsuccessfully in 0:08:36
[00:47:42] Makefile:58: recipe for target 'check' failed
[00:47:42] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1567a2c6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
