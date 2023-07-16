plain
[00:00:57] configure: rust.quiet-tests     := True
---
[00:41:03] .........................................................................i..........................
[00:41:09] ................i...................................................................................
---
[00:41:44] ............................................................................................i.......
[00:41:51] ................................................................i...................................
---
[00:42:29] ........FF.F........................................................................................
[00:42:46] .............................................i......................................................
---
[00:46:46] .............................i......................................................................
[00:47:01] ...............................................................i....................................
[00:47:16] ...............................................i....................................................
[00:47:37] ....................................................................................................
[00:47:58] ....................................................................................................
[00:48:20] ....................................................................................................
[00:48:46] .i................................................................................................i.
[00:49:12] .................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:49:22] ...................
[00:49:52] ....................................................F...............................................
[00:50:28] ...........F...................................................ii...................................
[00:51:14] ..........................i....................................................i.ii....test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:51:19] .............
[00:52:00] .......................................................................................iiiiiii......
---
[00:53:38] error: auxiliary build of "/checkout/src/test/run-pass/allocator/auxiliary/custom.rs" failed to compile:
[00:53:38] status: exit code: 101
[00:53:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator/auxiliary/custom.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:53:38] error[E0053]: method `alloc` has an incompatible type for trait
[00:53:38]   --> /checkout/src/test/run-pass/allocator/auxiliary/custom.rs:22:5
[00:53:38]    |
[00:53:38] 22 |     unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
[00:53:38]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::ptr::NonNull`, found *-ptr
[00:53:38]    |
[00:53:38]    = note: expected type `unsafe fn(&mut &'a A, std::heap::Layout) -> std::result::Result<std::ptr::NonNull<u8>, std::heap::AllocErr>`
[00:53:38]               found type `unsafe fn(&mut &'a A, std::heap::Layout) -> std::result::Result<*mut u8, std::heap::AllocErr>`
[00:53:38]
[00:53:38] error[E0053]: method `dealloc` has an incompatible type for trait
[00:53:38]   --> /checkout/src/test/run-pass/allocator/auxiliary/custom.rs:27:5
[00:53:38]    |
[00:53:38] 27 |     unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
[00:53:38]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::ptr::NonNull`, found *-ptr
[00:53:38]    |
[00:53:38]    = note: expected type `unsafe fn(&mut &'a A, std::ptr::NonNull<u8>, std::heap::Layout)`
[00:53:38]               found type `unsafe fn(&mut &'a A, *mut u8, std::heap::Layout)`
---
[00:53:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator/custom.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:53:38] error[E0053]: method `alloc` has an incompatible type for trait
[00:53:38]   --> /checkout/src/test/run-pass/allocator/custom.rs:26:5
[00:53:38]    |
[00:53:38] 26 |     unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
[00:53:38]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::ptr::NonNull`, found *-ptr
[00:53:38]    |
[00:53:38]    = note: expected type `unsafe fn(&mut &'a A, std::heap::Layout) -> std::result::Result<std::ptr::NonNull<u8>, std::heap::AllocErr>`
[00:53:38]               found type `unsafe fn(&mut &'a A, std::heap::Layout) -> std::result::Result<*mut u8, std::heap::AllocErr>`
[00:53:38]
[00:53:38] error[E0053]: method `dealloc` has an incompatible type for trait
[00:53:38]   --> /checkout/src/test/run-pass/allocator/custom.rs:31:5
[00:53:38]    |
[00:53:38] 31 |     unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
[00:53:38]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::ptr::NonNull`, found *-ptr
[00:53:38]    |
[00:53:38]    = note: expected type `unsafe fn(&mut &'a A, std::ptr::NonNull<u8>, std::heap::Layout)`
[00:53:38]               found type `unsafe fn(&mut &'a A, *mut u8, std::heap::Layout)`
---
[00:53:38] thread '[run-pass] run-pass/allocator/custom.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:53:38]
[00:53:38] ---- [run-pass] run-pass/allocator/xcrate-use2.rs stdout ----
[00:53:38]
[00:53:38] error: auxiliary build of "/checkout/src/test/run-pass/allocator/auxiliary/custom.rs" failed to compile:
[00:53:38] status: exit code: 101
[00:53:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator/auxiliary/custom.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:53:38] error[E0053]: method `alloc` has an incompatible type for trait
[00:53:38]   --> /checkout/src/test/run-pass/allocator/auxiliary/custom.rs:22:5
[00:53:38]    |
[00:53:38] 22 |     unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
[00:53:38]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::ptr::NonNull`, found *-ptr
[00:53:38]    |
[00:53:38]    = note: expected type `unsafe fn(&mut &'a A, std::heap::Layout) -> std::result::Result<std::ptr::NonNull<u8>, std::heap::AllocErr>`
[00:53:38]               found type `unsafe fn(&mut &'a A, std::heap::Layout) -> std::result::Result<*mut u8, std::heap::AllocErr>`
[00:53:38]
[00:53:38] error[E0053]: method `dealloc` has an incompatible type for trait
[00:53:38]   --> /checkout/src/test/run-pass/allocator/auxiliary/custom.rs:27:5
[00:53:38]    |
[00:53:38] 27 |     unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
[00:53:38]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::ptr::NonNull`, found *-ptr
[00:53:38]    |
[00:53:38]    = note: expected type `unsafe fn(&mut &'a A, std::ptr::NonNull<u8>, std::heap::Layout)`
[00:53:38]               found type `unsafe fn(&mut &'a A, *mut u8, std::heap::Layout)`
---
[00:53:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/realloc-16687.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/realloc-16687.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/realloc-16687.stage2-x86_64-unknown-linux-gnu.aux"
---
[00:53:38] 48 |     unsafe fn allocate(layout: Layout) -> *mut u8 {
[00:53:38]    |                                           ------- expected `*mut u8` because of return type
[00:53:38] ...
[00:53:38] 59 |         ret
[00:53:38]    |         ^^^ expected *-ptr, found struct `std::ptr::NonNull`
[00:53:38]    |
[00:53:38]    = note: expected type `*mut u8`
[00:53:38]               found type `std::ptr::NonNull<u8>`
[00:53:38]
[00:53:38] error[E0308]: mismatched types
[00:53:38]   --> /checkout/src/test/run-pass/realloc-16687.rs:67:22
[00:53:38]    |
[00:53:38] 67 |         Heap.dealloc(ptr, layout);
[00:53:38]    |                      ^^^ expected struct `std::ptr::NonNull`, found *-ptr
[00:53:38]    |
[00:53:38]    = note: expected type `std::ptr::NonNull<u8>`
[00:53:38]               found type `*mut u8`
[00:53:38]
[00:53:38] error[E0308]: mismatched types
[00:53:38]   --> /checkout/src/test/run-pass/realloc-16687.rs:75:32
[00:53:38]    |
[00:53:38] 75 |         let ret = Heap.realloc(ptr, old.clone(), new.clone())
[00:53:38]    |                                ^^^ expected struct `std::ptr::NonNull`, found *-ptr
[00:53:38]    |
[00:53:38]    = note: expected type `std::ptr::NonNull<u8>`
[00:53:38]               found type `*mut u8`
[00:53:38]
[00:53:38] error[E0308]: mismatched types
[00:53:38]   --> /checkout/src/test/run-pass/realloc-16687.rs:82:9
[00:53:38]    |
[00:53:38] 70 |     unsafe fn reallocate(ptr: *mut u8, old: Layout, new: Layout) -> *mut u8 {
[00:53:38]    |                                                                     ------- expected `*mut u8` because of return type
[00:53:38] ...
[00:53:38] 82 |         ret
[00:53:38]    |         ^^^ expected *-ptr, found struct `std::ptr::NonNull`
[00:53:38]    |
[00:53:38]    = note: expected type `*mut u8`
[00:53:38]               found type `std::ptr::NonNull<u8>`
---
[00:53:38] thread '[run-pass] run-pass/realloc-16687.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:53:38]
[00:53:38] ---- [run-pass] run-pass/regions-mock-trans.rs stdout ----
[00:53:38]
[00:53:38] error: compilation failed!
[00:53:38] status: exit code: 101
[00:53:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/regions-mock-trans.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-mock-trans.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-mock-trans.stage2-x86_64-unknown-linux-gnu.aux"
[00:5android-cross-path" "" "--color" "always"
[00:53:38] expected success, got: exit code: 101
[00:53:38]
[00:53:38]
[00:53:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:38] Build completed unsuccessfully in 0:13:21
[00:53:38] make: *** [check] Error 1
[00:53:38] Makefile:58: recipe for target 'check' failed
---
56708 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08
56704 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08/s-ezrcnlytcg-801bkp-c5w3u8gtr0ww
---
34544 ./obj/build/x86_64-unknown-linux-gnu/stage1-russtart:08d98f40
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:08d98f40:start=1522722454529367810,finish=1522722454536399606,duration=7031796
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23154f42
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:23154f42:start=1522722454542682585,finish=1522722454549008861,duration=6326276
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03eb59e4
$ dmesg | grep -i kill
[   10.980154] init: failsafe main process (1095) killed by TERM signal
