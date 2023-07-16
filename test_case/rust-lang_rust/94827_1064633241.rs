plain
Successfully built 6bf19b62f72f
Successfully tagged rust-ci:latest
Built container sha256:6bf19b62f72f55e1db28a8a9d7349ca533d2c3040616abe98f0cdc2e6e89a38a
Uploading finished image to https://ci-caches.rust-lang.org/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0
upload failed: - to s3://rust-lang-ci-sccache2/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---

144 LL |         unsafe { intrinsics::offset(self, count) }
145    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
146    |                  |
-    |                  pointer arithmetic failed: 0x0 is not a valid pointer
+    |                  pointer arithmetic failed: null pointer is not a valid pointer
148    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
150   ::: $DIR/offset_ub.rs:22:50


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/offset_ub.stderr
To only update this specific test, also pass `--test-args consts/offset_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:8:46
   |
   |
LL | pub const BEFORE_START: *const u8 = unsafe { (&0u8 as *const u8).offset(-1) }; //~NOTE
   |                                              ------------------------------ inside `BEFORE_START` at /checkout/src/test/ui/consts/offset_ub.rs:8:46
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc6 has size 1, so pointer to 2 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:9:43
   |
   |
LL | pub const AFTER_END: *const u8 = unsafe { (&0u8 as *const u8).offset(2) }; //~NOTE
   |                                           ----------------------------- inside `AFTER_END` at /checkout/src/test/ui/consts/offset_ub.rs:9:43
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc9 has size 100, so pointer to 101 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:10:45
   |
   |
LL | pub const AFTER_ARRAY: *const u8 = unsafe { [0u8; 100].as_ptr().offset(101) }; //~NOTE
   |                                             ------------------------------- inside `AFTER_ARRAY` at /checkout/src/test/ui/consts/offset_ub.rs:10:45
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u16>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:12:43
   |
   |
LL | pub const OVERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MAX) }; //~NOTE
   |                                           ------------------------------------- inside `OVERFLOW` at /checkout/src/test/ui/consts/offset_ub.rs:12:43
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u16>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:13:44
   |
   |
LL | pub const UNDERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MIN) }; //~NOTE
   |                                            ------------------------------------- inside `UNDERFLOW` at /checkout/src/test/ui/consts/offset_ub.rs:13:44
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:14:56
   |
   |
LL | pub const OVERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (usize::MAX as *const u8).offset(2) }; //~NOTE
   |                                                        ----------------------------------- inside `OVERFLOW_ADDRESS_SPACE` at /checkout/src/test/ui/consts/offset_ub.rs:14:56
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:15:57
   |
   |
LL | pub const UNDERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (1 as *const u8).offset(-2) }; //~NOTE
   |                                                         --------------------------- inside `UNDERFLOW_ADDRESS_SPACE` at /checkout/src/test/ui/consts/offset_ub.rs:15:57
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc24 has size 1, so pointer to 2 bytes starting at offset -4 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:16:49
   |
   |
LL | pub const NEGATIVE_OFFSET: *const u8 = unsafe { [0u8; 1].as_ptr().wrapping_offset(-2).offset(-2) }; //~NOTE
   |                                                 ------------------------------------------------ inside `NEGATIVE_OFFSET` at /checkout/src/test/ui/consts/offset_ub.rs:16:49
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc27 has size 0, so pointer to 1 byte starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:18:50
   |
   |
LL | pub const ZERO_SIZED_ALLOC: *const u8 = unsafe { [0u8; 0].as_ptr().offset(1) }; //~NOTE
   |                                                  --------------------------- inside `ZERO_SIZED_ALLOC` at /checkout/src/test/ui/consts/offset_ub.rs:18:50
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mut_ptr.rs:307:18
   |
LL |         unsafe { intrinsics::offset(self, count) as *mut T }
LL |         unsafe { intrinsics::offset(self, count) as *mut T }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  0x1 is not a valid pointer
   |                  inside `ptr::mut_ptr::<impl *mut u8>::offset` at /checkout/library/core/src/ptr/mut_ptr.rs:307:18
   |
  ::: /checkout/src/test/ui/consts/offset_ub.rs:19:42
   |
LL | pub const DANGLING: *const u8 = unsafe { ptr::NonNull::<u8>::dangling().as_ptr().offset(4) }; //~NOTE
   |                                          ------------------------------------------------- inside `DANGLING` at /checkout/src/test/ui/consts/offset_ub.rs:19:42
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: null pointer is not a valid pointer
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:22:50
   |
   |
LL | pub const NULL_OFFSET_ZERO: *const u8 = unsafe { ptr::null::<u8>().offset(0) }; //~NOTE
   |                                                  --------------------------- inside `NULL_OFFSET_ZERO` at /checkout/src/test/ui/consts/offset_ub.rs:22:50
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  0x7fffffffffffffff is not a valid pointer
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:25:47
   |
   |
LL | pub const UNDERFLOW_ABS: *const u8 = unsafe { (usize::MAX as *const u8).offset(isize::MIN) }; //~NOTE
   |                                               -------------------------------------------- inside `UNDERFLOW_ABS` at /checkout/src/test/ui/consts/offset_ub.rs:25:47
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
