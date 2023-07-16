plain

---- [ui] src/test/ui/const-ptr/forbidden_slices.rs stdout ----
diff of 64bit.stderr:

7    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
8    |         inside `std::slice::from_raw_parts::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:19:34
+   ::: $DIR/forbidden_slices.rs:18:34
11    |
11    |
12 LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };
-    |                                  ------------------------------ inside `S0` at $DIR/forbidden_slices.rs:19:34
+    |                                  ------------------------------ inside `S0` at $DIR/forbidden_slices.rs:18:34
15 error[E0080]: could not evaluate static initializer
16   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL

21    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
21    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
22    |         inside `std::slice::from_raw_parts::<()>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   ::: $DIR/forbidden_slices.rs:20:33
+   ::: $DIR/forbidden_slices.rs:19:33
25    |
25    |
26 LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };
-    |                                 ------------------------------ inside `S1` at $DIR/forbidden_slices.rs:20:33
+    |                                 ------------------------------ inside `S1` at $DIR/forbidden_slices.rs:19:33
29 error[E0080]: could not evaluate static initializer
30   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL


35    |         dereferencing pointer failed: ALLOC_ID has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
36    |         inside `std::slice::from_raw_parts::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:23:34
+   ::: $DIR/forbidden_slices.rs:22:34
39    |
39    |
40 LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };
-    |                                  ---------------------- inside `S2` at $DIR/forbidden_slices.rs:23:34
+    |                                  ---------------------- inside `S2` at $DIR/forbidden_slices.rs:22:34
43 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:26:1
+   --> $DIR/forbidden_slices.rs:25:1
45    |
45    |
46 LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) };
47    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
52            }
53 
54 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:28:1
-   --> $DIR/forbidden_slices.rs:28:1
+   --> $DIR/forbidden_slices.rs:27:1
56    |
57 LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, size_of::<&u32>()) };
58    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>: encountered a pointer, but expected plain (non-pointer) bytes
63            }
64 
65 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:30:1
-   --> $DIR/forbidden_slices.rs:30:1
+   --> $DIR/forbidden_slices.rs:29:1
67    |
68 LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) };
69    | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
74            }
75 
76 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:33:1
-   --> $DIR/forbidden_slices.rs:33:1
+   --> $DIR/forbidden_slices.rs:32:1
78    |
79 LL | pub static S7: &[u16] = unsafe {
80    | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)

93    |         dereferencing pointer failed: ALLOC_ID has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
94    |         inside `std::slice::from_raw_parts::<u64>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:44:5
+   ::: $DIR/forbidden_slices.rs:43:5
97    |
98 LL |     from_raw_parts(ptr, 1)
98 LL |     from_raw_parts(ptr, 1)
-    |     ---------------------- inside `S8` at $DIR/forbidden_slices.rs:44:5
+    |     ---------------------- inside `S8` at $DIR/forbidden_slices.rs:43:5
100 
101 error[E0080]: could not evaluate static initializer
102   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

112 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
113    |                                          ------------------------------ inside `from_ptr_range::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:47:34
+   ::: $DIR/forbidden_slices.rs:46:34
116    |
116    |
117 LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
-    |                                  ---------------------------------------- inside `R0` at $DIR/forbidden_slices.rs:47:34
+    |                                  ---------------------------------------- inside `R0` at $DIR/forbidden_slices.rs:46:34
120 error[E0080]: could not evaluate static initializer
121   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


131 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
132    |                                          ------------------------------ inside `from_ptr_range::<()>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:48:33
+   ::: $DIR/forbidden_slices.rs:47:33
135    |
135    |
136 LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
-    |                                 ---------------------------------------- inside `R1` at $DIR/forbidden_slices.rs:48:33
+    |                                 ---------------------------------------- inside `R1` at $DIR/forbidden_slices.rs:47:33
139    = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)
140 


150 LL |         unsafe { self.offset(count as isize) }
151    |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:51:25
+   ::: $DIR/forbidden_slices.rs:50:25
154    |
154    |
155 LL |     from_ptr_range(ptr..ptr.add(2))
-    |                         ---------- inside `R2` at $DIR/forbidden_slices.rs:51:25
+    |                         ---------- inside `R2` at $DIR/forbidden_slices.rs:50:25
158 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:53:1
+   --> $DIR/forbidden_slices.rs:52:1
160    |
160    |
161 LL | pub static R4: &[u8] = unsafe {
162    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
167            }
168 
169 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:58:1
-   --> $DIR/forbidden_slices.rs:58:1
+   --> $DIR/forbidden_slices.rs:57:1
171    |
172 LL | pub static R5: &[u8] = unsafe {
173    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>: encountered a pointer, but expected plain (non-pointer) bytes
178            }
179 
180 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:63:1
-   --> $DIR/forbidden_slices.rs:63:1
+   --> $DIR/forbidden_slices.rs:62:1
182    |
183 LL | pub static R6: &[bool] = unsafe {
184    | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
189            }
190 
191 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:68:1
-   --> $DIR/forbidden_slices.rs:68:1
+   --> $DIR/forbidden_slices.rs:67:1
193    |
194 LL | pub static R7: &[u16] = unsafe {
195    | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)

211 LL |         unsafe { self.offset(count as isize) }
212    |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:75:25
+   ::: $DIR/forbidden_slices.rs:74:25
215    |
215    |
216 LL |     from_ptr_range(ptr..ptr.add(1))
-    |                         ---------- inside `R8` at $DIR/forbidden_slices.rs:75:25
+    |                         ---------- inside `R8` at $DIR/forbidden_slices.rs:74:25
219 error[E0080]: could not evaluate static initializer
220   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


230 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
231    |                                          ------------------------------ inside `from_ptr_range::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:80:34
+   ::: $DIR/forbidden_slices.rs:79:34
234    |
234    |
235 LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
-    |                                  ----------------------------------------------- inside `R9` at $DIR/forbidden_slices.rs:80:34
+    |                                  ----------------------------------------------- inside `R9` at $DIR/forbidden_slices.rs:79:34
238 error[E0080]: could not evaluate static initializer
239   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


249 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
250    |                                          ------------------------------ inside `from_ptr_range::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:81:35
+   ::: $DIR/forbidden_slices.rs:80:35
253    |
253    |
254 LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };
-    |                                   ------------------------ inside `R10` at $DIR/forbidden_slices.rs:81:35
+    |                                   ------------------------ inside `R10` at $DIR/forbidden_slices.rs:80:35
257 error: aborting due to 18 previous errors
258 



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
   |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
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
   |         dereferencing pointer failed: alloc24 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
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
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>: encountered a pointer, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
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

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:32:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:32:1
   |
LL | pub static S7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾─────alloc68+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc82 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<u64>` at /checkout/library/core/src/slice/raw.rs:97:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:43:5
   |
LL |     from_raw_parts(ptr, 1)
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:43:5
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:43:5

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:745:18
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  |
   |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:745:18
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
  --> /checkout/library/core/src/ptr/const_ptr.rs:743:9
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
   |         |
   |         |
   |         the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', /checkout/library/core/src/ptr/const_ptr.rs:743:9
   |         inside `ptr::const_ptr::<impl *const ()>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:743:9
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
  --> /checkout/library/core/src/ptr/const_ptr.rs:457:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:457:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  out-of-bounds pointer arithmetic: alloc137 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u32>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:457:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add` at /checkout/library/core/src/ptr/const_ptr.rs:870:18
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
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>: encountered a pointer, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
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

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:67:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:67:1
   |
LL | pub static R7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾────alloc182+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:457:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  out-of-bounds pointer arithmetic: alloc199 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u64>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:457:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add` at /checkout/library/core/src/ptr/const_ptr.rs:870:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:74:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(1))
   |                         ---------- inside `R8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:74:25
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:745:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  ptr_offset_from_unsigned called on pointers into different allocations
   |                  ptr_offset_from_unsigned called on pointers into different allocations
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:745:18
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
  --> /checkout/library/core/src/ptr/const_ptr.rs:745:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  ptr_offset_from_unsigned called on pointers into different allocations
   |                  ptr_offset_from_unsigned called on pointers into different allocations
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:745:18
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
