plain
[00:45:46] ................................................................i...................................
[00:45:50] ....................................................................................................
[00:45:56] ....................................................................................................
[00:46:02] .............................................................................................i......
[00:46:05] ...........iiiiiiiii...................................................
[00:46:05] 
[00:46:05] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:46:54] ................................................................i...................................
[00:46:59] ....................................................................................................
[00:47:04] ....................................................................................................
[00:47:10] .............................................................................................i......
[00:47:13] ...........iiiiiiiii...................................................
[00:47:13] 
[00:47:13]  finished in 67.401
[00:47:13] travis_fold:end:test_ui_nll

---
[01:28:21] 
[01:28:21] failures:
[01:28:21] 
[01:28:21] ---- /checkout/src/doc/unstable-book/src/language-features/global-allocator.md - global_allocator (line 29) stdout ----
[01:28:21] error[E0053]: method `alloc` has an incompatible type for trait
[01:28:21]   --> /checkout/src/doc/unstable-book/src/language-features/global-allocator.md:38:5
[01:28:21]    |
[01:28:21] 10 |     unsafe fn alloc(&self, layout: Layout) -> *mut Opaque {
[01:28:21]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found *-ptr
[01:28:21]    |
[01:28:21]    = note: expected type `unsafe fn(&MyAllocator, std::heap::Layout) -> std::result::Result<std::ptr::NonNull<std::heap::Opaque>, std::heap::AllocErr>`
[01:28:21]               found type `unsafe fn(&MyAllocator, std::heap::Layout) -> *mut std::heap::Opaque`
[01:28:21] 
[01:28:21] error[E0053]: method `dealloc` has an incompatible type for trait
[01:28:21]   --> /checkout/src/doc/unstable-book/src/language-features/global-allocator.md:42:5
[01:28:21]    |
[01:28:21] 14 |     unsafe fn dealloc(&self, ptr: *mut Opaque, layout: Layout) {
[01:28:21]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::ptr::NonNull`, found *-ptr
[01:28:21]    |
[01:28:21]    = note: expected type `unsafe fn(&MyAllocator, std::ptr::NonNull<std::heap::Opaque>, std::heap::Layout)`
[01:28:21]               found type `unsafe fn(&MyAllocator, *mut std::heap::Opaque, std::heap::Layout)`
[01:28:21] thread '/checkout/src/doc/unstable-book/src/language-features/global-allocator.md - global_allocator (line 29)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:28:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:28:21] 
[01:28:21] 
---
[01:28:21] 
[01:28:21] 
[01:28:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:28:21] Build completed unsuccessfully in 0:44:54
[01:28:21] make: *** [check] Error 1
[01:28:21] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:010d4bb8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
