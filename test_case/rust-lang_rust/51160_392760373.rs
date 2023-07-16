plain
[00:48:38] ................................................................i...................................
[00:48:43] ....................................................................................................
[00:48:49] ....................................................................................................
[00:48:55] .............................................................................................i......
[00:48:58] ...........iiiiiiiii...................................................
[00:48:58] 
[00:48:58] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:49:48] ................................................................i...................................
[00:49:53] ....................................................................................................
[00:49:58] ....................................................................................................
[00:50:04] .............................................................................................i......
[00:50:07] ...........iiiiiiiii...................................................
[00:50:07] 
[00:50:07]  finished in 68.804
[00:50:07] travis_fold:end:test_ui_nll

---
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:50:07] 
[00:50:07] running 3015 tests
[00:50:21] ........FFF.........................................................................................
[00:50:54] ....................................................................................................
[00:51:09] ....................................................................................................
[00:51:22] ....................................................................................................
[00:51:44] ....................................................................................................
---
[00:59:16] ---- [run-pass] run-pass/allocator/custom.rs stdout ----
[00:59:16] 
[00:59:16] error: compilation failed!
[00:59:16] status: exit code: 101
[00:59:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator/custom.rs" "--target=x86_64-unknown-linux-gnu" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/auxiliary"
[00:59:16] ------------------------------------------
[00:59:16] 
[00:59:16] ------------------------------------------
[00:59:16] stderr:
[00:59:16] stderr:
[00:59:16] ------------------------------------------
[00:59:16] error[E0053]: method `alloc` has an incompatible type for trait
[00:59:16]   --> /checkout/src/test/run-pass/allocator/custom.rs:26:5
[00:59:16]    |
[00:59:16] 26 |     unsafe fn alloc(&self, layout: Layout) -> *mut Opaque {
[00:59:16]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found *-ptr
[00:59:16]    |
[00:59:16]    = note: expected type `unsafe fn(&A, std::heap::Layout) -> std::result::Result<std::ptr::NonNull<std::heap::Opaque>, std::heap::AllocErr>`
[00:59:16]               found type `unsafe fn(&A, std::heap::Layout) -> *mut std::heap::Opaque`
[00:59:16] 
[00:59:16] error[E0053]: method `dealloc` has an incompatible type for trait
[00:59:16]   --> /checkout/src/test/run-pass/allocator/custom.rs:31:5
[00:59:16]    |
[00:59:16] 31 |     unsafe fn dealloc(&self, ptr: *mut Opaque, layout: Layout) {
[00:59:16]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::ptr::NonNull`, found *-ptr
[00:59:16]    |
[00:59:16]    = note: expected type `unsafe fn(&A, std::ptr::NonNull<std::heap::Opaque>, std::heap::Layout)`
[00:59:16]               found type `unsafe fn(&A, *mut std::heap::Opaque, std::heap::Layout)`
[00:59:16] error: aborting due to 2 previous errors
[00:59:16] 
[00:59:16] For more information about this error, try `rustc --explain E0053`.
[00:59:16] 
[00:59:16] 
[00:59:16] ------------------------------------------
[00:59:16] 
[00:59:16] thread '[run-pass] run-pass/allocator/custom.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:59:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:16] 
[00:59:16] ---- [run-pass] run-pass/allocator/xcrate-use.rs stdout ----
[00:59:16] 
[00:59:16] error: auxiliary build of "/checkout/src/test/run-pass/allocator/auxiliary/custom.rs" failed to compile: 
[00:59:16] status: exit code: 101
[00:59:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator/auxiliary/custom.rs" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/auxiliary"
[00:59:16] ------------------------------------------
[00:59:16] 
[00:59:16] ------------------------------------------
[00:59:16] stderr:
[00:59:16] stderr:
[00:59:16] ------------------------------------------
[00:59:16] error[E0053]: method `alloc` has an incompatible type for trait
[00:59:16]   --> /checkout/src/test/run-pass/allocator/auxiliary/custom.rs:22:5
[00:59:16]    |
[00:59:16] 22 |     unsafe fn alloc(&self, layout: Layout) -> *mut Opaque {
[00:59:16]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found *-ptr
[00:59:16]    |
[00:59:16]    = note: expected type `unsafe fn(&A, std::heap::Layout) -> std::result::Result<std::ptr::NonNull<std::heap::Opaque>, std::heap::AllocErr>`
[00:59:16]               found type `unsafe fn(&A, std::heap::Layout) -> *mut std::heap::Opaque`
[00:59:16] 
[00:59:16] error[E0053]: method `dealloc` has an incompatible type for trait
[00:59:16]   --> /checkout/src/test/run-pass/allocator/auxiliary/custom.rs:27:5
[00:59:16]    |
[00:59:16] 27 |     unsafe fn dealloc(&self, ptr: *mut Opaque, layout: Layout) {
[00:59:16]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::ptr::NonNull`, found *-ptr
[00:59:16]    |
[00:59:16]    = note: expected type `unsafe fn(&A, std::ptr::NonNull<std::heap::Opaque>, std::heap::Layout)`
[00:59:16]               found type `unsafe fn(&A, *mut std::heap::Opaque, std::heap::Layout)`
[00:59:16] error: aborting due to 2 previous errors
[00:59:16] 
[00:59:16] For more information about this error, try `rustc --explain E0053`.
[00:59:16] 
[00:59:16] 
[00:59:16] ------------------------------------------
[00:59:16] 
[00:59:16] thread '[run-pass] run-pass/allocator/xcrate-use.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:59:16] 
[00:59:16] ---- [run-pass] run-pass/allocator/xcrate-use2.rs stdout ----
[00:59:16] 
[00:59:16] error: auxiliary build of "/checkout/src/test/run-pass/allocator/auxiliary/custom.rs" failed to compile: 
[00:59:16] status: exit code: 101
[00:59:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator/auxiliary/custom.rs" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2/auxiliary"
[00:59:16] ------------------------------------------
[00:59:16] 
[00:59:16] ------------------------------------------
[00:59:16] stderr:
[00:59:16] stderr:
[00:59:16] ------------------------------------------
[00:59:16] error[E0053]: method `alloc` has an incompatible type for trait
[00:59:16]   --> /checkout/src/test/run-pass/allocator/auxiliary/custom.rs:22:5
[00:59:16]    |
[00:59:16] 22 |     unsafe fn alloc(&self, layout: Layout) -> *mut Opaque {
[00:59:16]    |     ^^^^^^^^^87.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/realloc-16687/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/realloc-16687/auxiliary"
[00:59:16] ------------------------------------------
[00:59:16] 
[00:59:16] ------------------------------------------
[00:59:16] stderr:
[00:59:16] stderr:
[00:59:16] ------------------------------------------
[00:59:16] error[E0599]: no method named `as_opaque` found for type `std::ptr::NonNull<u8>` in the current scope
[00:59:16]   --> /checkout/src/test/run-pass/realloc-16687.rs:67:52
[00:59:16]    |
[00:59:16] 67 |         Global.dealloc(NonNull::new_unchecked(ptr).as_opaque(), layout);
[00:59:16] 
[00:59:16] 
[00:59:16] error[E0599]: no method named `as_opaque` found for type `std::ptr::NonNull<u8>` in the current scope
[00:59:16]   --> /checkout/src/test/run-pass/realloc-16687.rs:75:62
[00:59:16]    |
[00:59:16] 75 |         let ret = Global.realloc(NonNull::new_unchecked(ptr).as_opaque(), old.clone(), new.size())
[00:59:16] 
[00:59:16] error: aborting due to 2 previous errors
[00:59:16] 
[00:59:16] For more information about this error, try `rustc --explain E0599`.
---
[00:59:16] 
[00:59:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:59:16] 
[00:59:16] 
[00:59:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:16] 
[00:59:16] 
[00:59:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:16] Build completed unsuccessfully in 0:13:05
[00:59:16] Build completed unsuccessfully in 0:13:05
[00:59:16] Makefile:58: recipe for target 'check' failed
[00:59:16] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:33c72396
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
