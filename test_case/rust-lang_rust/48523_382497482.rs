plain
[00:54:19] ....i.................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:54:26] ..............
[00:54:59] ....................................................................................................
[00:55:30] ......................................................................ii............................
[00:56:22] .................................i....................................................i.ii...test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:57:08] ..............................................................................................iiiiii
[00:57:37] i...................................................................................................
[00:58:08] ....................................................................................................
[00:58:33] ....................................................................................................
---
travis_time:start:test_codegen-units
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:06] 
[01:03:06] running 39 tests
[01:03:09] i.......i..........F...........i.......
[01:03:09] 
[01:03:09] ---- [codegen-units] codegen-units/item-collection/trait-method-default-impl.rs stdout ----
[01:03:09]  
[01:03:09]  
[01:03:09] These items should have been contained but were not:
[01:03:09] 
[01:03:09] TRANS_ITEM fn trait_method_default_impl::SomeGenericTrait[0]::foo[0]<i32, u64>
[01:03:09] TRANS_ITEM fn trait_method_default_impl::SomeTrait[0]::foo[0]<i8>
[01:03:09] 
[01:03:09] 
[01:03:09] thread '[codegen-units] codegen-units/item-collection/trait-method-default-impl.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2232:13
[01:03:09] 
[01:03:09] 
[01:03:09] failures:
[01:03:09]     [codegen-units] codegen-units/item-collection/trait-method-default-impl.rs
[01:03:09]     [codegen-units] codegen-units/item-collection/trait-method-default-impl.rs
[01:03:09] 
[01:03:09] test result: FAILED. 35 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
[01:03:09] 
[01:03:09] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:03:09] 
[01:03:09] 
[01:03:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:09] 
[01:03:09] 
[01:03:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:09] Build completed unsuccessfully in 0:18:14
[01:03:09] Build completed unsuccessfully in 0:18:14
[01:03:09] make: *** [check] Error 1
[01:03:09] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11ec3c70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
