plain
[00:47:38] ....................................................................................................
[00:47:41] ....................................................................................................
[00:47:43] ....i...............................................................................................
[00:47:46] ....................................................................................................
[00:47:49] .....................................................iiiiiiiii......................................
[00:47:54] ....................................................................................................
[00:47:58] ....................................................................................................
[00:48:01] ..................................i.................................................................
[00:48:04] ....................................................................................i.i..ii.........
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:49] 
[00:55:49] running 97 tests
[00:57:38] ........................................F..........................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[00:59:02] failures:
[00:59:02] 
[00:59:02] ---- [run-pass] run-pass-fulldeps/macros2.rs stdout ----
[00:59:02] 
[00:59:02] 
[00:59:02] error: compilation failed!
[00:59:02] status: exit code: 1
[00:59:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/macros2.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macros2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macros2/auxiliary"
[00:59:02] ------------------------------------------
[00:59:02] 
[00:59:02] ------------------------------------------
[00:59:02] stderr:
[00:59:02] stderr:
[00:59:02] ------------------------------------------
[00:59:02] error: cannot find macro `d!` in this scope
[00:59:02]    |
[00:59:02]    |
[00:59:02] 50 | d!(df);
[00:59:02]    | ^ help: you could try the macro: `m`
[00:59:02] error: aborting due to previous error
[00:59:02] 
[00:59:02] 
[00:59:02] ------------------------------------------
---
[00:59:02] 
[00:59:02] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:59:02] 
[00:59:02] 
[00:59:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:02] 
[00:59:02] 
[00:59:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:02] Build completed unsuccessfully in 0:15:23
[00:59:02] Build completed unsuccessfully in 0:15:23
[00:59:02] make: *** [check] Error 1
[00:59:02] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1cb2216a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
