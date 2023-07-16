plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:47:07] 
[00:47:07] running 1464 tests
[00:47:11] .......................................................F.................................i..........
[00:47:20] ....................................................................................................
[00:47:24] ....................................................................................................
[00:47:27] ....................................................................................................
[00:47:30] ....................................................................................................
---
[00:47:54] .........................................................i..........................................
[00:47:58] ............................................................................ii......................
[00:48:04] ....................................................................................................
[00:48:10] ......................................................................................i.............
[00:48:13] ....iiiiiiiii...................................................
[00:48:13] 
[00:48:13] ---- [ui] ui/break-while-condition.rs stdout ----
[00:48:13] 
[00:48:13] 
[00:48:13] error: /checkout/src/test/ui/break-while-condition.rs:26: unexpected error: '26:13: 28:14: mismatched types [E0308]'
[00:48:13] 
[00:48:13] error: /checkout/src/test/ui/break-while-condition.rs:34: unexpected error: '34:13: 36:14: mismatched types [E0308]'
[00:48:13] 
[00:48:13] error: /checkout/src/test/ui/break-while-condition.rs:25: expected error not found: mismatched types
[00:48:13] 
[00:48:13] error: /checkout/src/test/ui/break-while-condition.rs:33: expected error not found: mismatched types
[00:48:13] 
[00:48:13] error: 2 unexpected errors found, 2 expected errors not found
[00:48:13] status: exit code: 101
[00:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/break-whil 
[00:48:13]     [ui] ui/break-while-condition.rs
[00:48:13] 
[00:48:13] test result: FAILED. 1447 passed; 1 failed; 16 ignored; 0 measured; 0 filtered out
[00:48:13] 
[00:48:13] 
[00:48:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:48:13] 
[00:48:13] 
[00:48:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:13] 
[00:48:13] 
[00:48:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:13] Build completed unsuccessfully in 0:02:40
[00:48:13] Build completed unsuccessfully in 0:02:40
[00:48:13] Makefile:58: recipe for target 'check' failed
[00:48:13] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06748ff0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
