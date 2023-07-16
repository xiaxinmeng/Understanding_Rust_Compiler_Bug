plain
---- [ui] src/test/ui/const-ptr/forbidden_slices.rs stdout ----
diff of 64bit.stderr:

7    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
8    |         inside `std::slice::from_raw_parts::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:18:34
+   ::: $DIR/forbidden_slices.rs:16:34
11    |
11    |
12 LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };
-    |                                  ------------------------------ inside `S0` at $DIR/forbidden_slices.rs:18:34
+    |                                  ------------------------------ inside `S0` at $DIR/forbidden_slices.rs:16:34
15 error[E0080]: could not evaluate static initializer
16   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL

21    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
21    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
22    |         inside `std::slice::from_raw_parts::<()>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:19:33
+   ::: $DIR/forbidden_slices.rs:17:33
25    |
25    |
26 LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };
-    |                                 ------------------------------ inside `S1` at $DIR/forbidden_slices.rs:19:33
+    |                                 ------------------------------ inside `S1` at $DIR/forbidden_slices.rs:17:33
29 error[E0080]: could not evaluate static initializer
30   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL


35    |         dereferencing pointer failed: ALLOC_ID has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
36    |         inside `std::slice::from_raw_parts::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:22:34
+   ::: $DIR/forbidden_slices.rs:20:34
39    |
39    |
40 LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };
-    |                                  ---------------------- inside `S2` at $DIR/forbidden_slices.rs:22:34
+    |                                  ---------------------- inside `S2` at $DIR/forbidden_slices.rs:20:34
43 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:25:1
+   --> $DIR/forbidden_slices.rs:23:1
45    |
45    |
46 LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) };
47    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
52            }
53 
54 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:27:1
-   --> $DIR/forbidden_slices.rs:27:1
+   --> $DIR/forbidden_slices.rs:25:1
56    |
57 LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, size_of::<&u32>()) };
58    | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
64            }
65 
66 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:29:1
-   --> $DIR/forbidden_slices.rs:29:1
+   --> $DIR/forbidden_slices.rs:27:1
68    |
69 LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) };
70    | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
75            }
76 
77 error[E0080]: it is undefined behavior to use this value
77 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:32:1
+   --> $DIR/forbidden_slices.rs:30:1
79    |
80 LL | pub static S7: &[u16] = unsafe {
81    | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)

94    |         dereferencing pointer failed: ALLOC_ID has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
95    |         inside `std::slice::from_raw_parts::<u64>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:43:5
+   ::: $DIR/forbidden_slices.rs:41:5
98    |
99 LL |     from_raw_parts(ptr, 1)
99 LL |     from_raw_parts(ptr, 1)
-    |     ---------------------- inside `S8` at $DIR/forbidden_slices.rs:43:5
+    |     ---------------------- inside `S8` at $DIR/forbidden_slices.rs:41:5
101 
102 error[E0080]: could not evaluate static initializer
103   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

113 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
114    |                                          ------------------------------ inside `from_ptr_range::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:46:34
+   ::: $DIR/forbidden_slices.rs:44:34
117    |
117    |
118 LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
-    |                                  ---------------------------------------- inside `R0` at $DIR/forbidden_slices.rs:46:34
+    |                                  ---------------------------------------- inside `R0` at $DIR/forbidden_slices.rs:44:34
121 error[E0080]: could not evaluate static initializer
122   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


132 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
133    |                                          ------------------------------ inside `from_ptr_range::<()>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:47:33
+   ::: $DIR/forbidden_slices.rs:45:33
136    |
136    |
137 LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
-    |                                 ---------------------------------------- inside `R1` at $DIR/forbidden_slices.rs:47:33
+    |                                 ---------------------------------------- inside `R1` at $DIR/forbidden_slices.rs:45:33
140    = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)
141 


151 LL |         unsafe { self.offset(count as isize) }
152    |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:50:25
+   ::: $DIR/forbidden_slices.rs:48:25
155    |
155    |
156 LL |     from_ptr_range(ptr..ptr.add(2))
-    |                         ---------- inside `R2` at $DIR/forbidden_slices.rs:50:25
+    |                         ---------- inside `R2` at $DIR/forbidden_slices.rs:48:25
159 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:52:1
+   --> $DIR/forbidden_slices.rs:50:1
161    |
161    |
162 LL | pub static R4: &[u8] = unsafe {
163    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
168            }
169 
170 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:57:1
-   --> $DIR/forbidden_slices.rs:57:1
+   --> $DIR/forbidden_slices.rs:55:1
172    |
173 LL | pub static R5: &[u8] = unsafe {
174    | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
180            }
181 
182 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:62:1
-   --> $DIR/forbidden_slices.rs:62:1
+   --> $DIR/forbidden_slices.rs:60:1
184    |
185 LL | pub static R6: &[bool] = unsafe {
186    | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
191            }
192 
193 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:67:1
-   --> $DIR/forbidden_slices.rs:67:1
+   --> $DIR/forbidden_slices.rs:65:1
195    |
196 LL | pub static R7: &[u16] = unsafe {
197    | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)

213 LL |         unsafe { self.offset(count as isize) }
214    |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:74:25
+   ::: $DIR/forbidden_slices.rs:72:25
217    |
217    |
218 LL |     from_ptr_range(ptr..ptr.add(1))
-    |                         ---------- inside `R8` at $DIR/forbidden_slices.rs:74:25
+    |                         ---------- inside `R8` at $DIR/forbidden_slices.rs:72:25
221 error[E0080]: could not evaluate static initializer
222   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


232 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
233    |                                          ------------------------------ inside `from_ptr_range::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:79:34
+   ::: $DIR/forbidden_slices.rs:77:34
236    |
236    |
237 LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
-    |                                  ----------------------------------------------- inside `R9` at $DIR/forbidden_slices.rs:79:34
+    |                                  ----------------------------------------------- inside `R9` at $DIR/forbidden_slices.rs:77:34
240 error[E0080]: could not evaluate static initializer
241   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


251 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
252    |                                          ------------------------------ inside `from_ptr_range::<u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:80:35
+   ::: $DIR/forbidden_slices.rs:78:35
255    |
255    |
256 LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };
-    |                                   ------------------------ inside `R10` at $DIR/forbidden_slices.rs:80:35
+    |                                   ------------------------ inside `R10` at $DIR/forbidden_slices.rs:78:35
259 error: aborting due to 18 previous errors
260 



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
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:16:34
   |
LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                  ------------------------------ inside `S0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:16:34
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
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:17:33
   |
LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                 ------------------------------ inside `S1` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:17:33
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc24 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<u32>` at /checkout/library/core/src/slice/raw.rs:97:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:20:34
   |
   |
LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };
   |                                  ---------------------- inside `S2` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:20:34
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:23:1
   |
   |
LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:25:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:25:1
   |
LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, size_of::<&u32>()) }; //~ ERROR: it is undefined behavior to use t...
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:27:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:27:1
   |
LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:30:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:30:1
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
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:41:5
   |
LL |     from_raw_parts(ptr, 1)
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:41:5
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:41:5

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:768:18
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:768:18
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<u32>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:44:34
   |
   |
LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                  ---------------------------------------- inside `R0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:44:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:766:9
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
   |         |
   |         |
   |         the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', /checkout/library/core/src/ptr/const_ptr.rs:766:9
   |         inside `ptr::const_ptr::<impl *const ()>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:766:9
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<()>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:45:33
   |
   |
LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                 ---------------------------------------- inside `R1` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:45:33
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:458:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:458:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  out-of-bounds pointer arithmetic: alloc137 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u32>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:458:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add` at /checkout/library/core/src/ptr/const_ptr.rs:894:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:48:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(2))
   |                         ---------- inside `R2` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:48:25
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:50:1
   |
   |
LL | pub static R4: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:55:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:55:1
   |
LL | pub static R5: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:60:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:60:1
   |
LL | pub static R6: &[bool] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:65:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:65:1
   |
LL | pub static R7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾────alloc182+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:458:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  out-of-bounds pointer arithmetic: alloc199 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u64>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:458:18
...
LL |         unsafe { self.offset(count as isize) }
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add` at /checkout/library/core/src/ptr/const_ptr.rs:894:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:72:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(1))
   |                         ---------- inside `R8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:72:25
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:768:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  `ptr_offset_from_unsigned` called on pointers into different allocations
   |                  `ptr_offset_from_unsigned` called on pointers into different allocations
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:768:18
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<u32>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:77:34
   |
   |
LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
   |                                  ----------------------------------------------- inside `R9` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:77:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:768:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  `ptr_offset_from_unsigned` called on pointers into different allocations
   |                  `ptr_offset_from_unsigned` called on pointers into different allocations
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:768:18
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<u32>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:78:35
   |
   |
LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };
   |                                   ------------------------ inside `R10` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:78:35
error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
