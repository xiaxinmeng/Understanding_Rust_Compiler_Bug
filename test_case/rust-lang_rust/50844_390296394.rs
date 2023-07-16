plain
[01:10:14] ................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[01:10:40] ....................................................................
[01:10:58] ....................................................................................................
[01:11:59] ....ii..............................................................i...............................
[01:12:05] .....................i.ii..test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[01:13:10] .............................iiiiiii................................................................
[01:13:38] ....................................................................................................
[01:14:01] ....................................................................................................
[01:14:53] ....................................................................................................
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:40] 
[01:19:40] running 90 tests
[01:21:36] ..F.......................................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:26:24] failures:
[01:26:24] 
[01:26:24] ---- [run-pass] run-pass-fulldeps/compiler-calls.rs stdout ----
[01:26:24] 
[01:26:24] 
[01:26:24] error: compilation failed!
[01:26:24] status: exit code: 101
[01:26:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/compiler-calls.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/compiler-calls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/compiler-calls/auxiliary"
[01:26:24] ------------------------------------------
[01:26:24] 
[01:26:24] ------------------------------------------
[01:26:24] stderr:
[01:26:24] stderr:
[01:26:24] ------------------------------------------
[01:26:24] error[E0368]: binary assignment operation `*=` cannot be applied to type `&mut u32`
[01:26:24]    |
[01:26:24]    |
[01:26:24] 46 |         self.count *= 2;
[01:26:24]    |         ----------^^^^^
[01:26:24]    |         |
[01:26:24]    |         cannot use `*=` on type `&mut u32`
[01:26:24]    |
[01:26:24]    = help: `*=` can be used on 'u32', you can dereference `self.count`: `*self.count`
[01:26:24] 
[01:26:24] error[E0368]: binary assignment operation `*=` cannot be applied to type `&mut u32`
[01:26:24]    |
[01:26:24]    |
[01:26:24] 59 |         self.count *= 3;
[01:26:24]    |         ----------^^^^^
[01:26:24]    |         |
[01:26:24]    |         cannot use `*=` on type `&mut u32`
[01:26:24]    |
[01:26:24]    = help: `*=` can be used on 'u32', you can dereference `self.count`: `*self.count`
[01:26:24] 
[01:26:24] error[E0368]: binary assignment operation `*=` cannot be applied to type `&mut u32`
[01:26:24]    |
[01:26:24]    |
[01:26:24] 65 |         self.count *= 5;
[01:26:24]    |         ----------^^^^^
[01:26:24]    |         |
[01:26:24]    |         cannot use `*=` on type `&mut u32`
[01:26:24]    |
[01:26:24]    = help: `*=` can be used on 'u32', you can dereference `self.count`: `*self.count`
[01:26:24] error: aborting due to 3 previous errors
[01:26:24] 
[01:26:24] For more information about this error, try `rustc --explain E0368`.
[01:26:24] 
---
[01:26:24] 
[01:26:24] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:26:24] 
[01:26:24] 
[01:26:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:26:24] 
[01:26:24] 
[01:26:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:24] Build completed unsuccessfully in 0:27:49
[01:26:24] Build completed unsuccessfully in 0:27:49
[01:26:24] make: *** [check] Error 1
[01:26:24] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0132f136
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
