plain
........................................................................................ 2376/13782
........................................................................................ 2464/13782
..............................................................F......................... 2552/13782
...................................................i.................................... 2640/13782
...............................................................ii...FF.F...F............ 2728/13782
F....................................................................................... 2816/13782
...................................................................i.i.................. 2992/13782
.........i.........F.......i.............................i.............................. 3080/13782
....................................i..............................F.................... 3168/13782
...i.................................................................................... 3256/13782
---
................................i.ii.................................................... 13728/13782
......................................................
failures:

---- [ui] src/test/ui/const-ptr/forbidden_slices.rs stdout ----
normalized 32bit.little.stderr:
error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |         inside `std::slice::from_raw_parts::<'_, u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
  ::: $DIR/forbidden_slices.rs:19:34
   |
   |
LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                  ------------------------------ inside `S0` at $DIR/forbidden_slices.rs:19:34
error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |         inside `std::slice::from_raw_parts::<'_, ()>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
  ::: $DIR/forbidden_slices.rs:20:33
   |
   |
LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                 ------------------------------ inside `S1` at $DIR/forbidden_slices.rs:20:33
error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: ALLOC_ID has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<'_, u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
  ::: $DIR/forbidden_slices.rs:23:34
   |
   |
LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };
   |                                  ---------------------- inside `S2` at $DIR/forbidden_slices.rs:23:34
error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:26:1
   |
   |
LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) };
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─ALLOC_ID─╼ 01 00 00 00                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:28:1
   |
   |
LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, size_of::<&u32>()) };
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─ALLOC_ID─╼ 04 00 00 00                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:30:1
   |
   |
LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) };
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─ALLOC_ID─╼ 04 00 00 00                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:33:1
   |
   |
LL | pub static S7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─A_ID+0x1─╼ 04 00 00 00                         │ ╾──╼....

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: ALLOC_ID has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<'_, u64>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
  ::: $DIR/forbidden_slices.rs:44:5
   |
LL |     from_raw_parts(ptr, 1)
   |     ---------------------- inside `S8` at $DIR/forbidden_slices.rs:44:5
   |     ---------------------- inside `S8` at $DIR/forbidden_slices.rs:44:5

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
  ::: $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
  ::: $DIR/forbidden_slices.rs:47:34
   |
   |
LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                  ---------------------------------------- inside `R0` at $DIR/forbidden_slices.rs:47:34
error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
   |         |
   |         |
   |         the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |         inside `ptr::const_ptr::<impl *const ()>::sub_ptr` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
  ::: $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, ()>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
  ::: $DIR/forbidden_slices.rs:48:33
   |
   |
LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                 ---------------------------------------- inside `R1` at $DIR/forbidden_slices.rs:48:33
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  out-of-bounds pointer arithmetic: ALLOC_ID has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u32>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
  ::: $DIR/forbidden_slices.rs:51:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(2))
   |                         ---------- inside `R2` at $DIR/forbidden_slices.rs:51:25
error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:53:1
   |
   |
LL | pub static R4: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾ALLOC_ID─╼ 01 00 00 00                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:58:1
   |
   |
LL | pub static R5: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾ALLOC_ID─╼ 04 00 00 00                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:63:1
   |
   |
LL | pub static R6: &[bool] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾ALLOC_ID─╼ 04 00 00 00                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:68:1
   |
   |
LL | pub static R7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾A_ID+0x1─╼ 04 00 00 00                         │ ╾──╼....

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  out-of-bounds pointer arithmetic: ALLOC_ID has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u64>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
  ::: $DIR/forbidden_slices.rs:75:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(1))
   |                         ---------- inside `R8` at $DIR/forbidden_slices.rs:75:25
error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  `ptr_offset_from_unsigned` called on pointers into different allocations
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
  ::: $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
  ::: $DIR/forbidden_slices.rs:80:34
   |
   |
LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
   |                                  ----------------------------------------------- inside `R9` at $DIR/forbidden_slices.rs:80:34
error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  `ptr_offset_from_unsigned` called on pointers into different allocations
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
  ::: $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
  ::: $DIR/forbidden_slices.rs:81:35
   |
   |
LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };
   |                                   ------------------------ inside `R10` at $DIR/forbidden_slices.rs:81:35
error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0080`.




The actual 32bit.little.stderr differed from the expected 32bit.little.stderr.
Actual 32bit.little.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices/forbidden_slices.32bit.little.stderr
To only update this specific test, also pass `--test-args const-ptr/forbidden_slices.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-ptr/forbidden_slices.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices/auxiliary"
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
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:34
   |
   |
LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                  ------------------------------ inside `S0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:34
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
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:20:33
   |
LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                 ------------------------------ inside `S1` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:20:33
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:100:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc18 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:100:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:23:34
   |
   |
LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };
   |                                  ---------------------- inside `S2` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:23:34
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:26:1
   |
   |
LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:28:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:28:1
   |
LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, size_of::<&u32>()) }; //~ ERROR: it is undefined behavior to use t...
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:30:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:30:1
   |
LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:33:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:33:1
   |
LL | pub static S7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─a50+0x1─╼ 04 00 00 00                         │ ╾──╼....

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:100:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc61 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<'_, u64>` at /checkout/library/core/src/slice/raw.rs:100:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:44:5
   |
LL |     from_raw_parts(ptr, 1)
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:44:5
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:44:5

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:773:18
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:773:18
   |
   |
  ::: /checkout/library/core/src/slice/raw.rs:227:42
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:227:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:34
   |
   |
LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                  ---------------------------------------- inside `R0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:771:9
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
   |         |
   |         |
   |         the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', /checkout/library/core/src/ptr/const_ptr.rs:771:9
   |         inside `ptr::const_ptr::<impl *const ()>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:771:9
  ::: /checkout/library/core/src/slice/raw.rs:227:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, ()>` at /checkout/library/core/src/slice/raw.rs:227:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:48:33
   |
   |
LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                 ---------------------------------------- inside `R1` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:48:33
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:461:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:461:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  out-of-bounds pointer arithmetic: alloc107 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u32>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:461:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add` at /checkout/library/core/src/ptr/const_ptr.rs:897:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:51:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(2))
   |                         ---------- inside `R2` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:51:25
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:53:1
   |
   |
LL | pub static R4: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:58:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:58:1
   |
LL | pub static R5: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:63:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:63:1
   |
LL | pub static R6: &[bool] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:68:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:68:1
   |
LL | pub static R7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾a134+0x1─╼ 04 00 00 00                         │ ╾──╼....

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:461:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  out-of-bounds pointer arithmetic: alloc145 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u64>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:461:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add` at /checkout/library/core/src/ptr/const_ptr.rs:897:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:75:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(1))
   |                         ---------- inside `R8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:75:25
