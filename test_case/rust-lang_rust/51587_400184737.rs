plain
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:44] 
[01:01:44] running 94 tests
[01:03:34] ........................................F................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:06:47] 
[01:06:47] test result: FAILED. 93 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:47] 
[01:06:47] 
[01:06:47] 
[01:06:47] 
[01:06:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:47] 
[01:06:47] 
[01:06:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:47] Build completed unsuTue, 26 Jun 2018 05:37:15 GMT
