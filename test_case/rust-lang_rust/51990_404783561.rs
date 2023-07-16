plain
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:47:11] 
[00:47:11] running 1563 tests
[00:47:14] ..................................................................................................i.
[00:47:18] ......................................................F.........i...................................
[00:47:23] ....................................................................................................
[00:47:25] ....................................................................................................
[00:47:27] ....................................................................................................
[00:47:30] ....................................................................................................
---
[00:47:58] ---- [ui (nll)] ui/const-eval/union_promotion.rs stdout ----
[00:47:58] 
[00:47:58] error: ui test compiled successfully!
[00:47:58] status: exit code: 0
[00:47:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/union_promotion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/union_promotion.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/union_promotion.nll/auxiliary" "-A" "unused"
[00:47:58] ------------------------------------------
[00:47:58] 
[00:47:58] ------------------------------------------
[00:47:58] stderr:
---
[00:47:58] 
[00:47:58] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:47:58] 
[00:47:58] 
[00:47:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:47:58] 
[00:47:58] 
[00:47:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:58] Build completed unsuccessfully in 0:02:15
[00:47:58] Build completed unsuccessfully in 0:02:15
[00:47:59] make: *** [check] Error 1
[00:47:59] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2ea79884
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
