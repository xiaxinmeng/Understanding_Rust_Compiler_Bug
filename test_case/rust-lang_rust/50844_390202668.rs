plain
[00:51:00] .................................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:51:01] ...
[00:51:14] ....................................................................................................
[00:51:53] ii..............................................................i...................................
[00:52:18] .................i.ii.........................................................................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:52:45] .........................iiiiiii....................................................................
[00:53:04] ....................................................................................................
[00:53:17] ....................................................................................................
[00:53:33] ................................................................................................
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:15] 
[00:57:15] running 90 tests
[00:58:56] ..F.......................................................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:01:27] failures:
[01:01:27] 
[01:01:27] ---- [run-pass] run-pass-fulldeps/compiler-calls.rs stdout ----
[01:01:27] 
[01:01:27] 
[01:01:27] error: compilation failed!
[01:01:27] status: exit code: 101
[01:01:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/compiler-calls.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/compiler-calls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/compiler-calls/auxiliary"
[01:01:27] ------------------------------------------
[01:01:27] 
[01:01:27] ------------------------------------------
[01:01:27] stderr:
[01:01:27] stderr:
[01:01:27] ------------------------------------------
[01:01:27] error[E0382]: use of moved value: `tc.count`
[01:01:27]    |
[01:01:27]    |
[01:01:27] 93 |     rustc_driver::run_compiler(&args, Box::new(tc), None, None);
[01:01:27]    |                                                -- value moved here
[01:01:27] 94 |     assert_eq!(tc.count, 30);
[01:01:27]    |                ^^^^^^^^ value used here after move
[01:01:27]    |
[01:01:27]    = note: move occurs because `tc` has type `TestCalls`, which does not implement the `Copy` trait
[01:01:27] error: aborting due to previous error
[01:01:27] 
[01:01:27] For more information about this error, try `rustc --explain E0382`.
[01:01:27] 
---
[01:01:27] 
[01:01:27] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:01:27] 
[01:01:27] 
[01:01:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:27] 
[01:01:27] 
[01:01:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:27] Build completed unsuccessfully in 0:19:11
[01:01:27] Build completed unsuccessfully in 0:19:11
[01:01:27] Makefile:58: recipe for target 'check' failed
[01:01:27] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21ac5270
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
