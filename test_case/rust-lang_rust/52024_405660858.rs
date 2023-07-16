plain
[00:52:41] ....................................................................................................
[00:52:52] ....................................................................................................
[00:53:01] ....................................................................................................
[00:53:17] ....................................................................................................
[00:53:26] ..........F.........................................................................................
[00:53:50] ....................................................................................................
[00:54:00] ....................................................................................................
[00:54:11] ....................................................................................................
[00:54:19] ....................................................................................................
---
[00:57:38] ....................................................................................................
[00:57:48] ....................................................................................................
[00:57:57] ....................................................................................................
[00:58:08] ....................................................................................................
nericBound<T: Trait>: 'static;
[00:58:15]    |                               ^ unused type parameter
[00:58:15] 
[00:58:15] error[E0091]: type parameter `T` is unused
[00:58:15]   --> /checkout/src/test/run-pass/existential_type.rs:96:35
[00:58:15]    |
[00:58:15] 96 | existential type PartiallyDefined<T>: 'static;
[00:58:15]    |                                   ^ unused type parameter
[00:58:15] 
[00:58:15] error[E0091]: type parameter `T` is unused
[00:58:15]    --> /checkout/src/test/run-pass/existential_type.rs:104:36
[00:58:15]     |
[00:58:15] 104 | existential type PartiallyDefined2<T>: 'static;
[00:58:15]     |                                    ^ unused type parameter
[00:58:15] error: aborting due to 3 previous errors
[00:58:15] 
[00:58:15] For more information about this error, try `rustc --explain E0091`.
[00:58:15] 
[00:58:15] 
[00:58:15] ------------------------------------------
[00:58:15] 
[00:58:15] thread '[run-pass] run-pass/existential_type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:58:15] 
[00:58:15] 
[00:58:15] failures:
[00:58:15]     [run-pass] run-pass/existential_type.rs
[00:58:15]     [run-pass] run-pass/existential_type.rs
[00:58:15] 
[00:58:15] test result: FAILED. 3036 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out
[00:58:15] 
[00:58:15] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:58:15] 
[00:58:15] 
[00:58:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:15] 
[00:58:15] 
[00:58:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:15] Build completed unsuccessfully in 0:09:03
[00:58:15] Build completed unsuccessfully in 0:09:03
[00:58:15] Makefile:58: recipe for target 'check' failed
[00:58:15] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:011f3dba
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
