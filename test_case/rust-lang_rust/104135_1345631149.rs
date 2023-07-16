plain
normalized 64bit.le.stderr:
error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
note: inside `std::slice::from_raw_parts::<'_, u32>`
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
note: inside `S0`
  --> $DIR/forbidden_slices.rs:19:34
   |
   |
LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
note: inside `std::slice::from_raw_parts::<'_, ()>`
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
note: inside `S1`
  --> $DIR/forbidden_slices.rs:20:33
   |
   |
LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: ALLOC_ID has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |
note: inside `std::slice::from_raw_parts::<'_, u32>`
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
note: inside `S2`
  --> $DIR/forbidden_slices.rs:23:34
   |
   |
LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:26:1
   |
   |
LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) };
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────ALLOC_ID───────╼ 01 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:28:1
   |
   |
LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, size_of::<&u32>()) };
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────ALLOC_ID───────╼ 08 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:30:1
   |
   |
LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) };
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────ALLOC_ID───────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:33:1
   |
   |
LL | pub static S7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾─────ALLOC_ID+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: ALLOC_ID has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |
note: inside `std::slice::from_raw_parts::<'_, u64>`
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
note: inside `S8`
  --> $DIR/forbidden_slices.rs:44:5
   |
LL |     from_raw_parts(ptr, 1)
LL |     from_raw_parts(ptr, 1)
   |     ^^^^^^^^^^^^^^^^^^^^^^

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `from_ptr_range::<'_, u32>`
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
note: inside `R0`
  --> $DIR/forbidden_slices.rs:47:34
   |
   |
LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
note: inside `ptr::const_ptr::<impl *const ()>::sub_ptr`
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
note: inside `from_ptr_range::<'_, ()>`
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
note: inside `R1`
  --> $DIR/forbidden_slices.rs:48:33
   |
   |
LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: ALLOC_ID has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
note: inside `ptr::const_ptr::<impl *const u32>::offset`
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `ptr::const_ptr::<impl *const u32>::add`
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
LL |         unsafe { self.offset(count as isize) }
note: inside `R2`
  --> $DIR/forbidden_slices.rs:51:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(2))

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:53:1
   |
   |
LL | pub static R4: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────ALLOC_ID───────╼ 01 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:58:1
   |
   |
LL | pub static R5: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────ALLOC_ID───────╼ 08 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:63:1
   |
   |
LL | pub static R6: &[bool] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────ALLOC_ID───────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/forbidden_slices.rs:68:1
   |
   |
LL | pub static R7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾────ALLOC_ID+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: ALLOC_ID has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
note: inside `ptr::const_ptr::<impl *const u64>::offset`
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `ptr::const_ptr::<impl *const u64>::add`
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
LL |         unsafe { self.offset(count as isize) }
note: inside `R8`
  --> $DIR/forbidden_slices.rs:75:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(1))

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called on pointers into different allocations
note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `from_ptr_range::<'_, u32>`
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
note: inside `R9`
  --> $DIR/forbidden_slices.rs:80:34
   |
   |
LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };

error[E0080]: could not evaluate static initializer
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called on pointers into different allocations
note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
  --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `from_ptr_range::<'_, u32>`
  --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
note: inside `R10`
  --> $DIR/forbidden_slices.rs:81:35
   |
   |
LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };

error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.le.stderr differed from the expected 64bit.le.stderr.
Actual 64bit.le.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices/forbidden_slices.64bit.le.stderr
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
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |
note: inside `std::slice::from_raw_parts::<'_, u32>`
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
note: inside `S0`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:34
   |
   |
LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:100:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
note: inside `std::slice::from_raw_parts::<'_, ()>`
  --> /checkout/library/core/src/slice/raw.rs:100:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
note: inside `S1`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:20:33
   |
   |
LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:100:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: alloc18 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |
note: inside `std::slice::from_raw_parts::<'_, u32>`
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
note: inside `S2`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:23:34
   |
   |
LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:26:1
   |
   |
LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
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
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:30:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:30:1
   |
LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:33:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:33:1
   |
LL | pub static S7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾─────alloc50+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:100:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: alloc61 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |
note: inside `std::slice::from_raw_parts::<'_, u64>`
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
note: inside `S8`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:44:5
   |
LL |     from_raw_parts(ptr, 1)
LL |     from_raw_parts(ptr, 1)
   |     ^^^^^^^^^^^^^^^^^^^^^^

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:809:18
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
  --> /checkout/library/core/src/ptr/const_ptr.rs:809:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `from_ptr_range::<'_, u32>`
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
note: inside `R0`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:34
   |
   |
LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:807:9
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', /checkout/library/core/src/ptr/const_ptr.rs:807:9
note: inside `ptr::const_ptr::<impl *const ()>::sub_ptr`
  --> /checkout/library/core/src/ptr/const_ptr.rs:807:9
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
note: inside `from_ptr_range::<'_, ()>`
  --> /checkout/library/core/src/slice/raw.rs:227:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
note: inside `R1`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:48:33
   |
   |
LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:474:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:474:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: alloc107 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
note: inside `ptr::const_ptr::<impl *const u32>::offset`
  --> /checkout/library/core/src/ptr/const_ptr.rs:474:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `ptr::const_ptr::<impl *const u32>::add`
  --> /checkout/library/core/src/ptr/const_ptr.rs:933:18
   |
LL |         unsafe { self.offset(count as isize) }
note: inside `R2`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:51:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(2))

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:53:1
   |
   |
LL | pub static R4: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
---
68 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-nonnull.rs:50:1
+   --> $DIR/ub-nonnull.rs:51:1
70    |
71 LL | const NULL_FAT_PTR: NonNull<dyn Send> = unsafe {
72    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1


The actual 64bit.le.stderr differed from the expected 64bit.le.stderr.
Actual 64bit.le.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.64bit.le.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:19:30
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:19:30
   |
LL |     let out_of_bounds_ptr = &ptr[255]; //~ ERROR evaluation of constant value failed
   |                              ^^^^^^^^ dereferencing pointer failed: alloc11 has size 1, so pointer to 256 bytes starting at offset 0 is out-of-bounds
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:23:1
   |
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:25:1
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:25:1
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:33:36
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:33:36
   |
LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:42:1
   |
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 42, but expected something in the range 10..=30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               2a 00 00 00                                     │ *...

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:48:1
   |
   |
LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 20, but expected something less or equal to 10, or greater or equal to 30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:51:1
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:51:1
   |
LL | const NULL_FAT_PTR: NonNull<dyn Send> = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               00 00 00 00 00 00 00 00 ╾───────alloc26───────╼ │ ........╾──────╼

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0080`.
