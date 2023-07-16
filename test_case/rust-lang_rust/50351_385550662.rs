plain
[00:52:31] running 2310 tests
[00:52:37] ....................................................................................................
[00:52:43] ....................................................................................................
[00:52:49] ....................................................................................................
[00:52:56] ............................................F.......................................................
[00:53:09] ....................................................................................................
[00:53:16] .........i..............................i...........................................................
[00:53:22] ....................................................................................................
[00:53:28] ...........i........................................................................................
---
[00:55:28] 
[00:55:28] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:55:28] 
[00:55:28] 
[00:55:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:28] 
[00:55:28] 
[00:55:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:28] Build completed unsuccessfully in 0:16:13
[00:55:28] Build completed unsuccessfully in 0:16:13
[00:55:28] make: *** [check] Error 1
[00:55:28] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25f526e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
