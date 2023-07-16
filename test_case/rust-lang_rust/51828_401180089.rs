plain
[00:55:58] ....................................................................................................
[00:56:10] ....................................................................................................
[00:56:35] ..................................................................i.................................
[00:56:49] ...................................................................................................i
[00:57:03] .............................................F.........................................i............
[00:57:13] ....................................................test [run-pass] run-pass/issue-29227.rs has been running for over 60 seconds
[00:57:53] ....................................................................................................
[00:58:10] ....................................................................................................
[00:58:29] ...............................................................i....................................
[00:58:48] ..............................................................i.....................................
---
[01:02:28] ---- [run-pass] run-pass/issue-44056.rs stdout ----
[01:02:28] 
[01:02:28] error: compilation failed!
[01:02:28] status: exit code: 101
[01:02:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-44056.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-44056/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ctarget-feature=+avx" "-Copt-level=2" "-Clto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-44056/auxiliary"
[01:02:28] ------------------------------------------
[01:02:28] 
[01:02:28] ------------------------------------------
[01:02:28] stderr:
[01:02:28] stderr:
[01:02:28] ------------------------------------------
[01:02:28] error: -O and -C opt-level both provided
[01:02:28] 
[01:02:28] ------------------------------------------
[01:02:28] 
[01:02:28] thread '[run-pass] run-pass/issue-44056.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
---
[01:02:28] 
[01:02:28] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:02:28] 
[01:02:28] 
[01:02:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:28] 
[01:02:28] 
[01:02:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:28] Build completed unsuccessfully in 0:13:31
[01:02:28] Build completed unsuccessfully in 0:13:31
[01:02:28] Makefile:58: recipe for target 'check' failed
[01:02:28] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0013f302
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
