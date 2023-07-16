plain
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:25] 
[00:52:25] running 95 tests
[00:54:14] ..................................................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
elp: message: `"_a"` is not a valid identifier
[00:55:37] error: aborting due to previous error
[00:55:37] 
[00:55:37] 
[00:55:37] ------------------------------------------
---
[00:55:37] test result: FAILED. 94 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:55:37] 
[00:55:37] 
[00:55:37] 
[00:55:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:37] 
[00:55:37] 
[00:55:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:37] Build completed unsuccessfully in 0:13:43
[00:55:37] Build completed unsuccessfully in 0:13:43
[00:55:37] make: *** [check] Error 1
[00:55:37] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0666f117
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
