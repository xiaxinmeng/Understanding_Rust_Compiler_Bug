plain
........................................................................................ 13288/13288

failures:

---- [ui] src/test/ui/const-ptr/forbidden_slices.rs stdout ----


32 LL |         &*ptr::slice_from_raw_parts(data, len)
34    |         |
34    |         |
-    |         dereferencing pointer failed: alloc22 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
+    |         dereferencing pointer failed: alloc26 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
36    |         inside `std::slice::from_raw_parts::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
37    |
38   ::: $DIR/forbidden_slices.rs:19:34
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
48    |
48    |
49    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
50    = note: the raw bytes of the constant (size: 16, align: 8) {
+                ╾───────alloc42───────╼ 01 00 00 00 00 00 00 00 │ ╾──────╼........
52            }
53 
54 error[E0080]: it is undefined behavior to use this value
54 error[E0080]: it is undefined behavior to use this value

59    |
60    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
61    = note: the raw bytes of the constant (size: 16, align: 8) {
+                ╾───────alloc55───────╼ 08 00 00 00 00 00 00 00 │ ╾──────╼........
63            }
64 
65 error[E0080]: it is undefined behavior to use this value
65 error[E0080]: it is undefined behavior to use this value

70    |
71    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
72    = note: the raw bytes of the constant (size: 16, align: 8) {
+                ╾───────alloc65───────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
74            }
75 
76 error[E0080]: it is undefined behavior to use this value
76 error[E0080]: it is undefined behavior to use this value

86    |
87    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
88    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾─────alloc75+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
+                ╾─────alloc79+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
91 
92 error[E0080]: could not evaluate static initializer


95 LL |         &*ptr::slice_from_raw_parts(data, len)
97    |         |
97    |         |
-    |         dereferencing pointer failed: alloc92 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
+    |         dereferencing pointer failed: alloc96 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
99    |         inside `std::slice::from_raw_parts::<u64>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
100    |
101   ::: $DIR/forbidden_slices.rs:40:5
149 LL |         unsafe { intrinsics::offset(self, count) }
150    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
151    |                  |
151    |                  |
-    |                  pointer arithmetic failed: alloc142 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
+    |                  pointer arithmetic failed: alloc154 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
153    |                  inside `ptr::const_ptr::<impl *const u32>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
154 ...
155 LL |         unsafe { self.offset(count as isize) }
172    |
172    |
173    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
174    = note: the raw bytes of the constant (size: 16, align: 8) {
+                ╾──────alloc159───────╼ 01 00 00 00 00 00 00 00 │ ╾──────╼........
176            }
177 
178 error[E0080]: it is undefined behavior to use this value
178 error[E0080]: it is undefined behavior to use this value

187    |
188    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
189    = note: the raw bytes of the constant (size: 16, align: 8) {
+                ╾──────alloc175───────╼ 08 00 00 00 00 00 00 00 │ ╾──────╼........
191            }
192 
193 error[E0080]: it is undefined behavior to use this value
193 error[E0080]: it is undefined behavior to use this value

202    |
203    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
204    = note: the raw bytes of the constant (size: 16, align: 8) {
+                ╾──────alloc191───────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
206            }
207 
208 error[E0080]: it is undefined behavior to use this value
208 error[E0080]: it is undefined behavior to use this value

217    |
218    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
219    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾────alloc197+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
+                ╾────alloc209+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
222 
223 error[E0080]: could not evaluate static initializer

226 LL |         unsafe { intrinsics::offset(self, count) }
226 LL |         unsafe { intrinsics::offset(self, count) }
227    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
228    |                  |
-    |                  pointer arithmetic failed: alloc218 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
+    |                  pointer arithmetic failed: alloc230 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
230    |                  inside `ptr::const_ptr::<impl *const u64>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
231 ...
232 LL |         unsafe { self.offset(count as isize) }

251 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
252    |                                          ------------------------------ inside `from_ptr_range::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
253    |
-   ::: $DIR/forbidden_slices.rs:75:34
+   ::: $DIR/forbidden_slices.rs:76:34
255    |
256 LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
-    |                                  ----------------------------------------------- inside `R9` at $DIR/forbidden_slices.rs:75:34
+    |                                  ----------------------------------------------- inside `R9` at $DIR/forbidden_slices.rs:76:34
259 error[E0080]: could not evaluate static initializer
260   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


270 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
271    |                                          ------------------------------ inside `from_ptr_range::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
272    |
-   ::: $DIR/forbidden_slices.rs:76:35
+   ::: $DIR/forbidden_slices.rs:77:35
274    |
275 LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };
-    |                                   ------------------------ inside `R10` at $DIR/forbidden_slices.rs:76:35
+    |                                   ------------------------ inside `R10` at $DIR/forbidden_slices.rs:77:35
278 error: aborting due to 18 previous errors
279 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices/forbidden_slices.stderr
To only update this specific test, also pass `--test-args const-ptr/forbidden_slices.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-ptr/forbidden_slices.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices/auxiliary"
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
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:15:34
   |
LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                  ------------------------------ inside `S0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:15:34
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
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:16:33
   |
LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                 ------------------------------ inside `S1` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:16:33
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc26 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<u32>` at /checkout/library/core/src/slice/raw.rs:97:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:34
   |
   |
LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };
   |                                  ---------------------- inside `S2` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:34
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:22:1
   |
   |
LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:24:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:24:1
   |
LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, size_of::<&u32>()) }; //~ ERROR: it is undefined behavior to use t...
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>: encountered a pointer, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:26:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:26:1
   |
LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:29:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:29:1
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
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾─────alloc79+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc96 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<u64>` at /checkout/library/core/src/slice/raw.rs:97:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:40:5
   |
LL |     from_raw_parts(ptr, 1)
LL |     from_raw_parts(ptr, 1)
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:40:5
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
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:43:34
   |
   |
LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                  ---------------------------------------- inside `R0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:43:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:741:9
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
   |         |
   |         |
   |         the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', /checkout/library/core/src/ptr/const_ptr.rs:741:9
   |         inside `ptr::const_ptr::<impl *const ()>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:741:9
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<()>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:44:33
   |
   |
LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                 ---------------------------------------- inside `R1` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:44:33
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:455:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:455:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc154 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u32>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:455:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add` at /checkout/library/core/src/ptr/const_ptr.rs:868:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(2))
   |                         ---------- inside `R2` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:25
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:49:1
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
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:54:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:54:1
   |
LL | / pub static R5: &[u8] = unsafe {
LL | |     //~^ ERROR: it is undefined behavior to use this value
LL | |     let ptr = &D3 as *const &u32;
LL | |     from_ptr_range(ptr.cast()..ptr.add(1).cast())
LL | | };
   | |__^ type validation failed at .<deref>: encountered a pointer, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:59:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:59:1
   |
LL | / pub static R6: &[bool] = unsafe {
LL | |     //~^ ERROR: it is undefined behavior to use this value
LL | |     let ptr = &D0 as *const u32 as *const bool;
LL | |     from_ptr_range(ptr..ptr.add(4))
LL | | };
   | |__^ type validation failed at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:64:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:64:1
   |
LL | / pub static R7: &[u16] = unsafe {
LL | |     //~^ ERROR: it is undefined behavior to use this value
LL | |     let ptr = (&D2 as *const Struct as *const u16).byte_add(1);
LL | |     from_ptr_range(ptr..ptr.add(4))
LL | | };
   | |__^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾────alloc209+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:455:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc230 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u64>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:455:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add` at /checkout/library/core/src/ptr/const_ptr.rs:868:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:71:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(1))
   |                         ---------- inside `R8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:71:25
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
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:76:34
   |
   |
LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
   |                                  ----------------------------------------------- inside `R9` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:76:34
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
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:77:35
   |
   |
LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };
   |                                   ------------------------ inside `R10` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:77:35
error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
