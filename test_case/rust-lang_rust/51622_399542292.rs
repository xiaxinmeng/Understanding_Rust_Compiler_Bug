plain
[00:56:04] ............................................................i.......................................
[00:56:23] ...........................................................i........................................
[00:56:52] ......................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:56:55] ..............
[00:57:12] .....F..............................................................................................
[00:58:13] ................................................i.ii................................................
[00:58:46] ........................................................iiiiiii.....................................
[00:59:06] ....................................................................................................
[00:59:24] ....................................................................................................
---
[00:59:52] 
[00:59:52] ------------------------------------------
[00:59:52] stderr:
[00:59:52] ------------------------------------------
[00:59:52] thread 'main' panicked at 'assertion failed: i >= 0 && i <= 10 && i % 2 == 0', /checkout/src/test/run-pass/range_inclusive.rs:44:9
[00:59:52] 
[00:59:52] ------------------------------------------
[00:59:52] 
[00:59:52] thread '[run-pass] run-pass/range_inclusive.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
---
[00:59:52] 
[00:59:52] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:59:52] 
[00:59:52] 
[00:59:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:52] 
[00:59:52] 
[00:59:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:52] Build completed unsuccessfully in 0:13:07
[00:59:52] Build completed unsuccessfully in 0:13:07
[00:59:52] Makefile:58: recipe for target 'check' failed
[00:59:52] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d7dcc50
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
