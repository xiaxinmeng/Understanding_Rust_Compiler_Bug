plain
........................................................................................ 2640/13874
........................................................................................ 2728/13874
........................................................................................ 2816/13874
........................................................................................ 2904/13874
......F.........F....................................................................... 2992/13874
.................................................i...................................... 3168/13874
................i....................................................................... 3256/13874
........................................................................................ 3344/13874
..................................................................................iiiii. 3432/13874
---

74                ╾───────ALLOC_ID───────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
75            }
76 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:32:1
+ error[E0080]: could not evaluate static initializer
+   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
79    |
- LL | pub static S7: &[u16] = unsafe {
-    | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
+ LL |         &*ptr::slice_from_raw_parts(data, len)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |         |
+    |         accessing memory with alignment 1, but alignment 2 is required
+    |         accessing memory with alignment 1, but alignment 2 is required
+    |         inside `std::slice::from_raw_parts::<'_, u16>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
82    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾─────ALLOC_ID+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
+   ::: $DIR/forbidden_slices.rs:36:5
+    |
+ LL |     from_raw_parts(ptr, 4)
+    |     ---------------------- inside `S7` at $DIR/forbidden_slices.rs:36:5
+    |     ---------------------- inside `S7` at $DIR/forbidden_slices.rs:36:5
87 
88 error[E0080]: could not evaluate static initializer
89   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL

190                ╾──────ALLOC_ID───────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
191            }
192 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:67:1
+ error[E0080]: could not evaluate static initializer
+   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
195    |
- LL | pub static R7: &[u16] = unsafe {
-    | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
+ LL |         &*ptr::slice_from_raw_parts(data, len)
+    |         |
+    |         accessing memory with alignment 1, but alignment 2 is required
+    |         accessing memory with alignment 1, but alignment 2 is required
+    |         inside `std::slice::from_raw_parts::<'_, u16>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
+ ...
+ LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
+    |              ----------------------------------------------------------- inside `from_ptr_range::<'_, u16>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
198    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾────ALLOC_ID+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
+   ::: $DIR/forbidden_slices.rs:70:5
+    |
+    |
+ LL |     from_ptr_range(ptr..ptr.add(4))
+    |     ------------------------------- inside `R7` at $DIR/forbidden_slices.rs:70:5
204 error[E0080]: could not evaluate static initializer
205   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices/forbidden_slices.64bit.stderr
To only update this specific test, also pass `--test-args const-ptr/forbidden_slices.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-ptr/forbidden_slices.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: could not evaluate static initializer
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |         inside `std::slice::from_raw_parts::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:100:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:18:34
   |
   |
LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                  ------------------------------ inside `S0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:18:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:100:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |         inside `std::slice::from_raw_parts::<'_, ()>` at /checkout/library/core/src/slice/raw.rs:100:9
   |
   |
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:33
   |
LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                 ------------------------------ inside `S1` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:33
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:100:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc18 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:100:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:22:34
   |
   |
LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };
   |                                  ---------------------- inside `S2` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:22:34
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:25:1
   |
   |
LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:27:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:27:1
   |
LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, size_of::<&u32>()) }; //~ ERROR: it is undefined behavior to use t...
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:29:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:29:1
   |
LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:100:9
  --> /checkout/library/core/src/slice/raw.rs:100:9
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         accessing memory with alignment 1, but alignment 2 is required
   |         accessing memory with alignment 1, but alignment 2 is required
   |         inside `std::slice::from_raw_parts::<'_, u16>` at /checkout/library/core/src/slice/raw.rs:100:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:36:5
   |
LL |     from_raw_parts(ptr, 4)
   |     ---------------------- inside `S7` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:36:5
   |     ---------------------- inside `S7` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:36:5

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:100:9
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc61 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<'_, u64>` at /checkout/library/core/src/slice/raw.rs:100:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:43:5
   |
LL |     from_raw_parts(ptr, 1)
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:43:5
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:43:5

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:791:18
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:791:18
   |
   |
  ::: /checkout/library/core/src/slice/raw.rs:227:42
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:227:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:46:34
   |
   |
LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                  ---------------------------------------- inside `R0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:46:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:789:9
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
   |         |
   |         |
   |         the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', /checkout/library/core/src/ptr/const_ptr.rs:789:9
   |         inside `ptr::const_ptr::<impl *const ()>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:789:9
  ::: /checkout/library/core/src/slice/raw.rs:227:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, ()>` at /checkout/library/core/src/slice/raw.rs:227:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:33
   |
   |
LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                 ---------------------------------------- inside `R1` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:33
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:456:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:456:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  out-of-bounds pointer arithmetic: alloc107 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u32>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:456:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add` at /checkout/library/core/src/ptr/const_ptr.rs:915:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:50:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(2))
   |                         ---------- inside `R2` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:50:25
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:52:1
   |
   |
LL | pub static R4: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:57:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:57:1
   |
LL | pub static R5: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:62:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:62:1
   |
LL | pub static R6: &[bool] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:100:9
  --> /checkout/library/core/src/slice/raw.rs:100:9
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         accessing memory with alignment 1, but alignment 2 is required
   |         accessing memory with alignment 1, but alignment 2 is required
   |         inside `std::slice::from_raw_parts::<'_, u16>` at /checkout/library/core/src/slice/raw.rs:100:9
...
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |              ----------------------------------------------------------- inside `from_ptr_range::<'_, u16>` at /checkout/library/core/src/slice/raw.rs:227:14
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:70:5
   |
   |
LL |     from_ptr_range(ptr..ptr.add(4))
   |     ------------------------------- inside `R7` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:70:5
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:456:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  out-of-bounds pointer arithmetic: alloc145 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u64>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:456:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add` at /checkout/library/core/src/ptr/const_ptr.rs:915:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:74:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(1))
   |                         ---------- inside `R8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:74:25
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:791:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  `ptr_offset_from_unsigned` called on pointers into different allocations
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:791:18
   |
   |
  ::: /checkout/library/core/src/slice/raw.rs:227:42
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:227:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:79:34
   |
   |
LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
   |                                  ----------------------------------------------- inside `R9` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:79:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:791:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  `ptr_offset_from_unsigned` called on pointers into different allocations
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:791:18
   |
   |
  ::: /checkout/library/core/src/slice/raw.rs:227:42
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:227:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:80:35
   |
   |
LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };
   |                                   ------------------------ inside `R10` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:80:35
error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---

1 error[E0080]: evaluation of constant value failed
+   --> $DIR/copy-intrinsic.rs:20:5
+    |
+ LL |     copy_nonoverlapping(&src as *const _ as *const i32, &mut dst as *mut _ as *mut i32, 0);
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ accessing memory with alignment 1, but alignment 4 is required
+ error[E0080]: evaluation of constant value failed
2   --> $DIR/copy-intrinsic.rs:27:5
3    |
3    |
4 LL |     copy_nonoverlapping(0x100 as *const i32, dangle, 0);

22 LL |     copy_nonoverlapping(&x, &mut y, 1usize << (mem::size_of::<usize>() * 8 - 1));
23    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow computing total size of `copy_nonoverlapping`
- error: aborting due to 4 previous errors
+ error: aborting due to 5 previous errors
26 
27 For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/copy-intrinsic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/copy-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/copy-intrinsic" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/copy-intrinsic/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/copy-intrinsic.rs:20:5
   |
   |
LL |     copy_nonoverlapping(&src as *const _ as *const i32, &mut dst as *mut _ as *mut i32, 0);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ accessing memory with alignment 1, but alignment 4 is required
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/copy-intrinsic.rs:27:5
   |
   |
LL |     copy_nonoverlapping(0x100 as *const i32, dangle, 0); //~ ERROR evaluation of constant value failed [E0080]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ memory access failed: alloc5 has size 4, so pointer at offset 40 is out-of-bounds
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/copy-intrinsic.rs:34:5
   |
   |
LL |     copy_nonoverlapping(dangle, 0x100 as *mut i32, 0); //~ ERROR evaluation of constant value failed [E0080]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ memory access failed: alloc7 has size 4, so pointer at offset 40 is out-of-bounds
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/copy-intrinsic.rs:41:5
   |
   |
LL |     copy(&x, &mut y, 1usize << (mem::size_of::<usize>() * 8 - 1)); //~ ERROR evaluation of constant value failed [E0080]

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/copy-intrinsic.rs:47:5
   |
   |
LL |     copy_nonoverlapping(&x, &mut y, 1usize << (mem::size_of::<usize>() * 8 - 1)); //~ evaluation of constant value failed [E0080]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow computing total size of `copy_nonoverlapping`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/extra-const-ub/detect-extra-ub.rs#no_flag stdout ----

error in revision `no_flag`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "no_flag" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/extra-const-ub/detect-extra-ub.no_flag" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/extra-const-ub/detect-extra-ub.no_flag/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/library/core/src/ptr/mod.rs:1133:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         accessing memory with alignment 1, but alignment 4 is required
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:1133:9
   |
  ::: /checkout/library/core/src/ptr/const_ptr.rs:1199:18
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::const_ptr::<impl *const u32>::read` at /checkout/library/core/src/ptr/const_ptr.rs:1199:18
  ::: /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:38:9
   |
LL |         ptr.read();
   |         ---------- inside `INNER` at /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:38:9
   |         ---------- inside `INNER` at /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:38:9

note: erroneous constant used
  --> /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:32:5
   |
LL |     INNER; //[with_flag]~ constant

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
