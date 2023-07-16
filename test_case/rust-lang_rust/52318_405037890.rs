plain
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:45:20] 
[00:45:20] running 1567 tests
[00:45:24] ..................................................................................................i.
[00:45:28] .............................F..................................i...................................
[00:45:32] ....................................................................................................
[00:45:35] ....................................................................................................
[00:45:37] ....................................................................................................
[00:45:40] ....................................................................................................
---
[00:46:08] 
[00:46:08] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:46:08] 
[00:46:08] 
[00:46:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:46:08] 
[00:46:08] 
[00:46:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:08] Build completed unsuccessfully in 0:02:13
[00:46:08] Build completed unsuccessfully in 0:02:13
[00:46:08] make: *** [check] Error 1
[00:46:08] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0038e42b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
