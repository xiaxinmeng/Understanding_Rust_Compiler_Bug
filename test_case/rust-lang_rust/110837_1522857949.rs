plain
---- [ui] tests/ui/const-ptr/forbidden_slices.rs stdout ----
diff of stderr:

131    |
132    = note: out-of-bounds pointer arithmetic: allocN has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
- note: inside `ptr::const_ptr::<impl *const u32>::offset`
-   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
136 note: inside `ptr::const_ptr::<impl *const u32>::add`
137   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
137   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
138 note: inside `R2`

195    |
196    = note: out-of-bounds pointer arithmetic: allocN has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
- note: inside `ptr::const_ptr::<impl *const u64>::offset`
-   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
200 note: inside `ptr::const_ptr::<impl *const u64>::add`
201   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
---
To only update this specific test, also pass `--test-args const-ptr/forbidden_slices.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-ptr/forbidden_slices.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices/auxiliary"
stdout: none
error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:100:9
   |
   = note: dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   = note: dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |
note: inside `std::slice::from_raw_parts::<'_, u32>`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:100:9
note: inside `S0`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:19:34
   |
LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:100:9
   |
   |
   = note: dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |
note: inside `std::slice::from_raw_parts::<'_, ()>`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:100:9
note: inside `S1`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:20:33
   |
LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:100:9
   |
   |
   = note: dereferencing pointer failed: alloc18 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |
note: inside `std::slice::from_raw_parts::<'_, u32>`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:100:9
note: inside `S2`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:23:34
   |
LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:26:1
   |
   |
LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:28:1
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:28:1
   |
LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, size_of::<&u32>()) }; //~ ERROR: it is undefined behavior to use t...
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:30:1
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:30:1
   |
LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:33:1
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:33:1
   |
LL | pub static S7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[1]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾─────alloc50+0x2─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:100:9
   |
   |
   = note: dereferencing pointer failed: alloc58 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |
note: inside `std::slice::from_raw_parts::<'_, u64>`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:100:9
note: inside `S8`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:44:5
LL |     from_raw_parts(ptr, 1)
   |     ^^^^^^^^^^^^^^^^^^^^^^

error[E0080]: could not evaluate static initializer
error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:798:18
   |
   = note: out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |
note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:798:18
note: inside `from_ptr_range::<'_, u32>`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:227:42
note: inside `R0`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:47:34
   |
LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:796:9
   |
   |
   = note: the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:796:9
note: inside `ptr::const_ptr::<impl *const ()>::sub_ptr`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:796:9
note: inside `from_ptr_range::<'_, ()>`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:227:42
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:227:42
note: inside `R1`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:48:33
   |
LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:927:13
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:927:13
   |
   = note: out-of-bounds pointer arithmetic: alloc100 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
note: inside `ptr::const_ptr::<impl *const u32>::add`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:927:13
note: inside `R2`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:51:25
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:51:25
   |
LL |     from_ptr_range(ptr..ptr.add(2))

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:53:1
   |
   |
LL | pub static R4: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:58:1
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:58:1
   |
LL | pub static R5: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:63:1
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:63:1
   |
LL | pub static R6: &[bool] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:100:9
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:100:9
   |
   = note: accessing memory with alignment 1, but alignment 2 is required
   |
note: inside `std::slice::from_raw_parts::<'_, u16>`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:100:9
note: inside `from_ptr_range::<'_, u16>`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:227:14
note: inside `R7`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:70:5
   |
LL |     from_ptr_range(ptr..ptr.add(4)) //~ inside `R7`

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:927:13
   |
   |
   = note: out-of-bounds pointer arithmetic: alloc138 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
note: inside `ptr::const_ptr::<impl *const u64>::add`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:927:13
note: inside `R8`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:74:25
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:74:25
   |
LL |     from_ptr_range(ptr..ptr.add(1)) //~ inside `R8`

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:798:18
   |
   |
   = note: `ptr_offset_from_unsigned` called on pointers into different allocations
   |
note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:798:18
note: inside `from_ptr_range::<'_, u32>`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:227:42
note: inside `R9`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:79:34
   |
LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:798:18
   |
   |
   = note: `ptr_offset_from_unsigned` called on pointers into different allocations
   |
note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:798:18
note: inside `from_ptr_range::<'_, u32>`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/raw.rs:227:42
note: inside `R10`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:80:35
   |
LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };

error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0080`.
