plain
test [ui] src/test/ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] src/test/ui/const-ptr/forbidden_slices.rs stdout ----
diff of 32bit.stderr:
86    |
86    |
87    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
88    = note: the raw bytes of the constant (size: 8, align: 4) {
-                ╾─a79+0x1─╼ 04 00 00 00                         │ ╾──╼....
+                ╾─a49+0x1─╼ 04 00 00 00                         │ ╾──╼....
91 
92 error[E0080]: could not evaluate static initializer

217    |
217    |
218    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
219    = note: the raw bytes of the constant (size: 8, align: 4) {
-                ╾a209+0x1─╼ 04 00 00 00                         │ ╾──╼....
+                ╾a129+0x1─╼ 04 00 00 00                         │ ╾──╼....
222 
223 error[E0080]: could not evaluate static initializer



Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices/forbidden_slices.32bit.stderr
To only update this specific test, also pass `--test-args const-ptr/forbidden_slices.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-ptr/forbidden_slices.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: could not evaluate static initializer
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         dereferencing pointer failed: null pointer is not a valid pointer
   |         inside `std::slice::from_raw_parts::<u32>` at /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:18:34
   |
LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                  ------------------------------ inside `S0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:18:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         dereferencing pointer failed: null pointer is not a valid pointer
   |         inside `std::slice::from_raw_parts::<()>` at /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:33
   |
LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                 ------------------------------ inside `S1` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:33
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc16 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<u32>` at /checkout/library/core/src/slice/raw.rs:97:9
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
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:27:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:27:1
   |
LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, size_of::<&u32>()) }; //~ ERROR: it is undefined behavior to use t...
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>: encountered a pointer, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:29:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:29:1
   |
LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:32:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:32:1
   |
LL | / pub static S7: &[u16] = unsafe {
LL | |     //~^ ERROR: it is undefined behavior to use this value
LL | |     let ptr = (&D2 as *const Struct as *const u16).byte_add(1);
LL | |
LL | |     from_raw_parts(ptr, 4)
LL | | };
   | |__^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─a49+0x1─╼ 04 00 00 00                         │ ╾──╼....

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc61 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<u64>` at /checkout/library/core/src/slice/raw.rs:97:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:43:5
   |
LL |     from_raw_parts(ptr, 1)
LL |     from_raw_parts(ptr, 1)
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:43:5
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:743:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  out-of-bounds offset_from: null pointer is not a valid pointer
   |                  out-of-bounds offset_from: null pointer is not a valid pointer
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:743:18
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<u32>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:46:34
   |
   |
LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                  ---------------------------------------- inside `R0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:46:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:741:9
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
   |         |
   |         |
   |         the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', /rustc/91fae5f28eaffe13700eaa9004b9165cdf910d8a/library/core/src/ptr/const_ptr.rs:741:9
   |         inside `ptr::const_ptr::<impl *const ()>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:741:9
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<()>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:33
   |
   |
LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                 ---------------------------------------- inside `R1` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:33
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:455:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:455:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc104 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u32>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:455:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add` at /checkout/library/core/src/ptr/const_ptr.rs:868:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:50:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(2))
   |                         ---------- inside `R2` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:50:25
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:52:1
   |
   |
LL | / pub static R4: &[u8] = unsafe {
LL | |     //~^ ERROR: it is undefined behavior to use this value
LL | |     let ptr = (&D1) as *const MaybeUninit<&u32> as *const u8;
LL | |     from_ptr_range(ptr..ptr.add(1))
LL | | };
   | |__^ type validation failed at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:57:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:57:1
   |
LL | / pub static R5: &[u8] = unsafe {
LL | |     //~^ ERROR: it is undefined behavior to use this value
LL | |     let ptr = &D3 as *const &u32;
LL | |     from_ptr_range(ptr.cast()..ptr.add(1).cast())
LL | | };
   | |__^ type validation failed at .<deref>: encountered a pointer, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:62:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:62:1
   |
LL | / pub static R6: &[bool] = unsafe {
LL | |     //~^ ERROR: it is undefined behavior to use this value
LL | |     let ptr = &D0 as *const u32 as *const bool;
LL | |     from_ptr_range(ptr..ptr.add(4))
LL | | };
   | |__^ type validation failed at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:67:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:67:1
   |
LL | / pub static R7: &[u16] = unsafe {
LL | |     //~^ ERROR: it is undefined behavior to use this value
LL | |     let ptr = (&D2 as *const Struct as *const u16).byte_add(1);
LL | |     from_ptr_range(ptr..ptr.add(4))
LL | | };
   | |__^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾a129+0x1─╼ 04 00 00 00                         │ ╾──╼....

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:455:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc140 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u64>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:455:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add` at /checkout/library/core/src/ptr/const_ptr.rs:868:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:74:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(1))
   |                         ---------- inside `R8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:74:25
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:743:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  ptr_offset_from_unsigned cannot compute offset of pointers into different allocations.
   |                  ptr_offset_from_unsigned cannot compute offset of pointers into different allocations.
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:743:18
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<u32>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:79:34
   |
   |
LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
   |                                  ----------------------------------------------- inside `R9` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:79:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:743:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  ptr_offset_from_unsigned cannot compute offset of pointers into different allocations.
   |                  ptr_offset_from_unsigned cannot compute offset of pointers into different allocations.
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:743:18
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<u32>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:80:35
   |
   |
LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };
   |                                   ------------------------ inside `R10` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:80:35
error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
