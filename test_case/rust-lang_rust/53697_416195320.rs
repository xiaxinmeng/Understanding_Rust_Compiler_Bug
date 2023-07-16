plain
[00:44:44] ....................................................................................................
[00:44:46] ....................................................................................................
[00:44:49] ....................................................................................................
[00:44:53] ........i...........................................................................................
[00:44:57] ...............................................................................FFFF.F...............
[00:45:03] .................ii.iii.............................................................................
[00:45:06] ....................................................................................................
[00:45:08] ....................................................................................................
[00:45:10] ....................................................................................................
---
[00:46:36] ---- [ui] ui/consts/const-int-rotate.rs stdout ----
[00:46:36] 
[00:46:36] error: ui test compiled successfully!
[00:46:36] status: exit code: 0
[00:46:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-rotate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate/auxiliary" "-A" "unused"
[00:46:36] ------------------------------------------
[00:46:36] 
[00:46:36] ------------------------------------------
[00:46:36] stderr:
---
[00:46:36] ---- [ui] ui/consts/const-int-sign.rs stdout ----
[00:46:36] 
[00:46:36] error: ui test compiled successfully!
[00:46:36] status: exit code: 0
[00:46:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-sign.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-sign/a" "-Crp[00:46:36] 
[00:46:36] thread '[ui] ui/consts/const-int-wrapping.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:36] 
[00:46:36] failures:
[00:46:36]     [ui] ui/consts/const-int-conversion.rs
[00:46:36]     [ui] ui/consts/const-int-overflowing.rs
---
[00:46:36] 
[00:46:36] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:46:36] 
[00:46:36] 
[00:46:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:36] 
[00:46:36] 
[00:46:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:36] Build completed unsuccessfully in 0:03:02
[00:46:36] Build completed unsuccessfully in 0:03:02
[00:46:36] make: *** [check] Error 1
[00:46:36] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04dde61b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
