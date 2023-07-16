plain
[00:46:02] ......................................................................i.............................
[00:46:06] ....................................................................................................
[00:46:12] ....................................................................................................
[00:46:19] ....................................................................................................
[00:46:22] ..i.................iiiiiiiii...................................................
[00:46:22] 
[00:46:22] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:47:13] ......................................................................i.............................
[00:47:18] ....................................................................................................
[00:47:23] ....................................................................................................
[00:47:29] ....................................................................................................
[00:47:33] ..i.................iiiiiiiii...................................................
[00:47:33] 
[00:47:33]  finished in 70.534
[00:47:33] travis_fold:end:test_ui_nll

---
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:47:33] 
[00:47:33] running 3017 tests
[00:47:48] ........F...........................................................................................
[00:48:18] ....................................................................................................
[00:48:32] ....................................................................................................
[00:48:44] ....................................................................................................
[00:49:05] ....................................................................................................
---
[00:56:10] ---- [run-pass] run-pass/allocator/custom.rs stdout ----
[00:56:10] 
[00:56:10] error: compilation failed!
[00:56:10] status: exit code: 101
[00:56:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator/custom.rs" "--target=x86_64-unknown-linux-gnu" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/auxiliary"
[00:56:10] ------------------------------------------
[00:56:10] 
[00:56:10] ------------------------------------------
[00:56:10] stderr:
[00:56:10] stderr:
[00:56:10] ------------------------------------------
[00:56:10] error[E0599]: no method named `unwrap` found for type `*mut u8` in the current scope
[00:56:10]   --> /checkout/src/test/run-pass/allocator/custom.rs:48:41
[00:56:10]    |
[00:56:10] 48 |         let ptr = alloc(layout.clone()).unwrap();
[00:56:10] 
[00:56:10] 
[00:56:10] error[E0277]: the trait bound `std::fmt::Debug: std::marker::Sized` is not satisfied
[00:56:10]   --> /checkout/src/test/run-pass/allocator/custom.rs:48:13
[00:56:10]    |
[00:56:10] 48 |         let ptr = alloc(layout.clone()).unwrap();
[00:56:10]    |             ^^^ `std::fmt::Debug` does not have a constant size known at compile-time
[00:56:10]    |
[00:56:10]    = help: the trait `std::marker::Sized` is not implemented for `std::fmt::Debug`
[00:56:10]    = note: all local variables must have a statically known size
[00:56:10] error[E0308]: mismatched types
[00:56:10]   --> /checkout/src/test/run-pass/allocator/custom.rs:51:17
[00:56:10]    |
[00:56:10]    |
[00:56:10] 51 |         dealloc(ptr, layout.clone());
[00:56:10]    |                 ^^^ expected *-ptr, found trait std::fmt::Debug
[00:56:10]    |
[00:56:10]    = note: expected type `*mut u8`
[00:56:10]               found type `std::fmt::Debug`
[00:56:10] 
[00:56:10] error[E0599]: no method named `alloc` found for type `std::heap::System` in the current scope
[00:56:10]   --> /checkout/src/test/run-pass/allocator/custom.rs:60:26
[00:56:10]    |
[00:56:10] 60 |         let ptr = System.alloc(layout.clone()).unwrap();
[00:56:10]    |
[00:56:10]    = help: items from traits can only be used if the trait is in scope
[00:56:10] help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:56:10]    |
[00:56:10]    |
[00:56:10] 18 | use std::heap::GlobalAlloc;
[00:56:10]    |
[00:56:10] 
[00:56:10] error[E0277]: the trait bound `std::fmt::Debug: std::marker::Sized` is not satisfied
[00:56:10]   --> /checkout/src/test/run-pass/allocator/custom.rs:60:13
[00:56:10]    |
[00:56:10] 60 |         let ptr = System.alloc(layout.clone()).unwrap();
[00:56:10]    |             ^^^ `std::fmt::Debug` does not have a constant size known at compile-time
[00:56:10]    |
[00:56:10]    = help: the trait `std::marker::Sized` is not implemented for `std::fmt::Debug`
[00:56:10]    = note: all local variables must have a statically known size
[00:56:10] 
[00:56:10] error[E0599]: no method named `dealloc` found for type `std::heap::System` in the current scope
[00:56:10]   --> /checkout/src/test/run-pass/allocator/custom.rs:63:16
[00:56:10]    |
[00:56:10] 63 |         System.dealloc(ptr, layout);
[00:56:10]    |
[00:56:10]    = help: items from traits can only be used if the trait is in scope
[00:56:10] help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:56:10]    |
---
[00:56:10] 
[00:56:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:10] 
[00:56:10] 
[00:56:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:10] 
[00:56:10] 
[00:56:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:10] Build completed unsuccessfully in 0:12:30
[00:56:10] Build completed unsuccessfully in 0:12:30
[00:56:10] make: *** [check] Error 1
[00:56:10] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12c84752
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
