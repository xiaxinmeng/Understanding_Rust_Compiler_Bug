plain
Receiving objects: 100% (311/311), 49.40 KiB | 24.70 MiB/s, done.
---
Resolving deltas: 100% (252/252), completed with 77 local objects.
---
[00:00:00] Attempting with retry: sh -c rm -f download-src-llvm.tar.gz &&         curl -sSL -o download-src-llvm.tar.gz https://github.com/rust-lang/llvm/archive/7243155b1c3da0a980c868a87adebf00e0b33989.tar.gz
---
[00:00:49] configure: rust.quiet-tests     := True
---
[00:41:44] ..........................................................................i.........................
[00:41:50] .................i..................................................................................
---
[00:42:27] ............................................................................................i.......
[00:42:34] ................................................................i...................................
---
[00:43:13] .............F......................................................................................
[00:43:29] .............................................i......................................................
---
[00:47:21] .............................i......................................................................
[00:47:35] ..............................................................i.....................................
[00:47:51] ...............................................i....................................................
[00:48:12] ....................................................................................................
[00:48:34] ....................................................................................................
[00:48:56] ....................................................................................................
[00:49:22] .i...............................................................................................i..
[00:49:48] .................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:49:59] ...................
[00:50:30] ..................................................F.................................................
[00:51:07] .............................................................ii.....................................
[00:51:52] ........................i....................................................i.ii....test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:51:59] ...............
[00:52:40] .....................................................................................iiiiiii........
---
[00:54:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator/xcrate-use2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:54:17] error[E0599]: no method named `unwrap` found for type `*mut std::alloc::Void` in the current scope
[00:54:17]   --> /checkout/src/test/run-pass/allocator/xcrate-use2.rs:33:48
[00:54:17]    |
[00:54:17] 33 |         let ptr = Global.alloc(layout.clone()).unwrap();
[00:54:17]    |                                                ^^^^^^
[00:54:17]
[00:54:17] error[E0277]: the trait bound `std::fmt::Debug: std::marker::Sized` is not satisfied
[00:54:17]   --> /checkout/src/test/run-pass/allocator/xcrate-use2.rs:33:13
[00:54:17]    |
[00:54:17] 33 |         let ptr = Global.alloc(layout.clone()).unwrap();
[00:54:17]    |             ^^^ `std::fmt::Debug` does not have a constant size known at compile-time
[00:54:17]    |
[00:54:17]    = help: the trait `std::marker::Sized` is not implemented for `std::fmt::Debug`
[00:54:17]    = note: all local variables must have a statically known size
[00:54:17]
[00:54:17] error[E0308]: mismatched types
[00:54:17]   --> /checkout/src/test/run-pass/allocator/xcrate-use2.rs:36:24
[00:54:17]    |
[00:54:17] 36 |         Global.dealloc(ptr, layout.clone());
[00:54:17]    |                        ^^^ expected *-ptr, found trait std::fmt::Debug
[00:54:17]    |
[00:54:17]    = note: expected type `*mut std::alloc::Void`
[00:54:17]               found type `std::fmt::Debug`
---
[00:54:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/realloc-16687.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/realloc-16687.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/realloc-16687.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:54:17] warning: use of deprecated item 'std::alloc::Heap': type renamed to `Global`
[00:54:17]   --> /checkout/src/test/run-pass/realloc-16687.rs:18:17
[00:54:17]    |
[00:54:17] 18 | use std::heap::{Heap, Alloc, Layout};
[00:54:17]    |                 ^^^^
[00:54:17]    |
[00:54:17]    = note: #[warn(deprecated)] on by default
[00:54:17]
[00:54:17] warning: use of deprecated item 'std::alloc::Heap': type renamed to `Global`
[00:54:17]   --> /checkout/src/test/run-pass/realloc-16687.rs:53:19
[00:54:17]    |
[00:54:17] 53 |         let ret = Heap.alloc(layout.clone()).unwrap_or_else(|_| Heap.oom());
[00:54:17]    |                   ^^^^
[00:54:17]
[00:54:17] warning: use of deprecated item 'std::alloc::Heap': type renamed to `Global`
[00:54:17]   --> /checkout/src/test/run-pass/realloc-16687.rs:53:65
[00:54:17]    |
[00:54:17] 53 |         let ret = Heap.alloc(layout.clone()).unwrap_or_else(|_| Heap.oom());
[00:54:17]    |                                                                 ^^^^
[00:54:17]
[00:54:17] warning: use of deprecated item 'std::alloc::Heap': type renamed to `Global`
[00:54:17]   --> /checkout/src/test/run-pass/realloc-16687.rs:67:9
[00:54:17]    |
[00:54:17] 67 |         Heap.dealloc(NonNull::new_unchecked(ptr), layout);
[00:54:17]    |         ^^^^
[00:54:17]
[00:54:17] warning: use of deprecated item 'std::alloc::Heap': type renamed to `Global`
[00:54:17]   --> /checkout/src/test/run-pass/realloc-16687.rs:75:19
[00:54:17]    |
[00:54:17] 75 |         let ret = Heap.realloc(NonNull::new_unchecked(ptr), old.clone(), new.clone())
[00:54:17]    |                   ^^^^
[00:54:17]
[00:54:17] warning: use of deprecated item 'std::alloc::Heap': type renamed to `Global`
[00:54:17]   --> /checkout/src/test/run-pass/realloc-16687.rs:76:33
[00:54:17]    |
[00:54:17] 76 |             .unwrap_or_else(|_| Heap.oom());
[00:54:17]    |                                 ^^^^
[00:54:17]
[00:54:17] error[E0308]: mismatched types
[00:54:17]   --> /checkout/src/test/run-pass/realloc-16687.rs:59:9
[00:54:17]    |
[00:54:17] 48 |     unsafe fn allocate(layout: Layout) -> *mut u8 {
[00:54:17]    |                                           ------- expected `*mut u8` because of return type
[00:54:17] ...
[00:54:17] 59 |         ret.as_ptr()
[00:54:17]    |         ^^^^^^^^^^^^ expected u8, found extern type `std::alloc::Void`
[00:54:17]    |
[00:54:17]    = note: expected type `*mut u8`
[00:54:17]               found type `*mut std::alloc::Void`
[00:54:17]
[00:54:17] error[E0308]: mismatched types
[00:54:17]   --> /checkout/src/test/run-pass/realloc-16687.rs:67:45
[00:54:17]    |
[00:54:17] 67 |         Heap.dealloc(NonNull::new_unchecked(ptr), layout);
[00:54:17]    |                                             ^^^ expected extern type `std::alloc::Void`, found u8
[00:54:17]    |
[00:54:17]    = note: expected type `*mut std::alloc::Void`
[00:54:17]               found type `*mut u8`
[00:54:17]
[00:54:17] error[E0308]: mismatched types
[00:54:17]   --> /checkout/src/test/run-pass/realloc-16687.rs:75:55
[00:54:17]    |
[00:54:17] 75 |         let ret = Heap.realloc(NonNull::new_unchecked(ptr), old.clone(), new.clone())
[00:54:17]    |                                                       ^^^ expected extern type `std::alloc::Void`, found u8
[00:54:17]    |
[00:54:17]    = note: expected type `*mut std::alloc::Void`
[00:54:17]               found type `*mut u8`
[00:54:17]
[00:54:17] error[E0308]: mismatched types
[00:54:17]   --> /checkout/src/test/run-pass/realloc-16687.rs:75:74
[00:54:17]    |
[00:54:17] 75 |         let ret = Heap.realloc(NonNull::new_unchecked(ptr), old.clone(), new.clone())
[00:54:17]    |                                                                          ^^^^^^^^^^^ expected usize, found struct `std::alloc::Layout`
[00:54:17]    |
[00:54:17]    = note: expected type `usize`
[00:54:17]               found type `std::alloc::Layout`
[00:54:17]
[00:54:17] error[E0308]: mismatched types
[00:54:17]   --> /checkout/src/test/run-pass/realloc-16687.rs:82:9
[00:54:17]    |
[00:54:17] 70 |     unsafe fn reallocate(ptr: *mut u8, old: Layout, new: Layout) -> *mut u8 {
[00:54:17]    |                                                                     ------- expected `*mut u8` because of return type
[00:54:17] ...
[00:54:17] 82 |         ret.as_ptr()
[00:54:17]    |         ^^^^^^^^^^^^ expected u8, found extern type `std::alloc::Void`
[00:54:17]    |
[00:54:17]    = note: expected type `*mut u8`
[00:54:17]               found type `*mut std::alloc::Void`
---
[00:54:17] thread '[run-pass] run-pass/realloc-16687.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
---
[00:54:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:17] expected success, got: exit code: 101
[00:54:17]
[00:54:17]
[00:54:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:17] Build completed unsuccessfully in 0:13:47
[00:54:17] make: *** [check] Error 1
[00:54:17] Makefile:58: recipe for target 'check' failed
