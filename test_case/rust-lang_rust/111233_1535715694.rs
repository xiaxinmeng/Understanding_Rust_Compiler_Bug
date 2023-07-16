plain
---- [ui] tests/ui/const-ptr/out_of_bounds_read.rs stdout ----
diff of stderr:

31    |
32    = note: dereferencing pointer failed: alloc5 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
- note: inside `std::ptr::read::<u32>`
+ note: inside `ptr::read_mut::<u32>`
35   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
36 note: inside `ptr::mut_ptr::<impl *mut u32>::read`
---
To only update this specific test, also pass `--test-args const-ptr/out_of_bounds_read.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-ptr/out_of_bounds_read.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:1171:9
   |
   |
   = note: dereferencing pointer failed: alloc5 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
note: inside `std::ptr::read::<u32>`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:1171:9
note: inside `_READ`
  --> fake-test-src-base/const-ptr/out_of_bounds_read.rs:12:33
  --> fake-test-src-base/const-ptr/out_of_bounds_read.rs:12:33
   |
LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:1171:9
   |
   |
   = note: dereferencing pointer failed: alloc5 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
note: inside `std::ptr::read::<u32>`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:1171:9
note: inside `ptr::const_ptr::<impl *const u32>::read`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:1206:18
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:1206:18
note: inside `_CONST_READ`
  --> fake-test-src-base/const-ptr/out_of_bounds_read.rs:13:39
   |
LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:1187:9
   |
   |
   = note: dereferencing pointer failed: alloc5 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
note: inside `ptr::read_mut::<u32>`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:1187:9
note: inside `ptr::mut_ptr::<impl *mut u32>::read`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:1316:18
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:1316:18
note: inside `_MUT_READ`
  --> fake-test-src-base/const-ptr/out_of_bounds_read.rs:14:37
   |
LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
