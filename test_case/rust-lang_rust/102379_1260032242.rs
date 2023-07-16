plain
........................................................................................ 2376/13597
........................................................................................ 2464/13597
...F.................................................................................... 2552/13597
........................................................................................ 2640/13597
...........F.F.F.F.......F.............................................................. 2728/13597
........................................................................................ 2904/13597
...............................................F........................................ 2992/13597
..................................................................i..................... 3080/13597
.....F..........................i....................................................... 3168/13597
---
---- [ui] src/test/ui/const-ptr/forbidden_slices.rs stdout ----
diff of 64bit.stderr:

7    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
8    |         inside `std::slice::from_raw_parts::<'_, u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:18:34
+   ::: $DIR/forbidden_slices.rs:19:34
11    |
11    |
12 LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };
-    |                                  ------------------------------ inside `S0` at $DIR/forbidden_slices.rs:18:34
+    |                                  ------------------------------ inside `S0` at $DIR/forbidden_slices.rs:19:34
15 error[E0080]: could not evaluate static initializer
16   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL

21    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
21    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
22    |         inside `std::slice::from_raw_parts::<'_, ()>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:19:33
+   ::: $DIR/forbidden_slices.rs:20:33
25    |
25    |
26 LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };
-    |                                 ------------------------------ inside `S1` at $DIR/forbidden_slices.rs:19:33
+    |                                 ------------------------------ inside `S1` at $DIR/forbidden_slices.rs:20:33
29 error[E0080]: could not evaluate static initializer
30   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL


35    |         dereferencing pointer failed: ALLOC_ID has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
36    |         inside `std::slice::from_raw_parts::<'_, u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:22:34
+   ::: $DIR/forbidden_slices.rs:23:34
39    |
39    |
40 LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };
-    |                                  ---------------------- inside `S2` at $DIR/forbidden_slices.rs:22:34
+    |                                  ---------------------- inside `S2` at $DIR/forbidden_slices.rs:23:34
43 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:25:1
+   --> $DIR/forbidden_slices.rs:26:1
45    |
45    |
46 LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) };
47    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
52            }
53 
54 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:27:1
-   --> $DIR/forbidden_slices.rs:27:1
+   --> $DIR/forbidden_slices.rs:28:1
56    |
57 LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, size_of::<&u32>()) };
58    | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
64            }
65 
66 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:29:1
-   --> $DIR/forbidden_slices.rs:29:1
+   --> $DIR/forbidden_slices.rs:30:1
68    |
69 LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) };
70    | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
75            }
76 
77 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:32:1
-   --> $DIR/forbidden_slices.rs:32:1
+   --> $DIR/forbidden_slices.rs:33:1
79    |
80 LL | pub static S7: &[u16] = unsafe {
81    | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)

94    |         dereferencing pointer failed: ALLOC_ID has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
95    |         inside `std::slice::from_raw_parts::<'_, u64>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:43:5
+   ::: $DIR/forbidden_slices.rs:44:5
98    |
99 LL |     from_raw_parts(ptr, 1)
99 LL |     from_raw_parts(ptr, 1)
-    |     ---------------------- inside `S8` at $DIR/forbidden_slices.rs:43:5
+    |     ---------------------- inside `S8` at $DIR/forbidden_slices.rs:44:5
101 
102 error[E0080]: could not evaluate static initializer
103   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

113 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
114    |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:46:34
+   ::: $DIR/forbidden_slices.rs:47:34
117    |
117    |
118 LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
-    |                                  ---------------------------------------- inside `R0` at $DIR/forbidden_slices.rs:46:34
+    |                                  ---------------------------------------- inside `R0` at $DIR/forbidden_slices.rs:47:34
121 error[E0080]: could not evaluate static initializer
122   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


132 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
133    |                                          ------------------------------ inside `from_ptr_range::<'_, ()>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:47:33
+   ::: $DIR/forbidden_slices.rs:48:33
136    |
136    |
137 LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
-    |                                 ---------------------------------------- inside `R1` at $DIR/forbidden_slices.rs:47:33
+    |                                 ---------------------------------------- inside `R1` at $DIR/forbidden_slices.rs:48:33
140    = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)
141 


151 LL |         unsafe { self.offset(count as isize) }
152    |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:50:25
+   ::: $DIR/forbidden_slices.rs:51:25
155    |
155    |
156 LL |     from_ptr_range(ptr..ptr.add(2))
-    |                         ---------- inside `R2` at $DIR/forbidden_slices.rs:50:25
+    |                         ---------- inside `R2` at $DIR/forbidden_slices.rs:51:25
159 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:52:1
+   --> $DIR/forbidden_slices.rs:53:1
161    |
161    |
162 LL | pub static R4: &[u8] = unsafe {
163    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
168            }
169 
170 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:57:1
-   --> $DIR/forbidden_slices.rs:57:1
+   --> $DIR/forbidden_slices.rs:58:1
172    |
173 LL | pub static R5: &[u8] = unsafe {
174    | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
180            }
181 
182 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:62:1
-   --> $DIR/forbidden_slices.rs:62:1
+   --> $DIR/forbidden_slices.rs:63:1
184    |
185 LL | pub static R6: &[bool] = unsafe {
186    | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
191            }
192 
193 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:67:1
-   --> $DIR/forbidden_slices.rs:67:1
+   --> $DIR/forbidden_slices.rs:68:1
195    |
196 LL | pub static R7: &[u16] = unsafe {
197    | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)

213 LL |         unsafe { self.offset(count as isize) }
214    |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:74:25
+   ::: $DIR/forbidden_slices.rs:75:25
217    |
217    |
218 LL |     from_ptr_range(ptr..ptr.add(1))
-    |                         ---------- inside `R8` at $DIR/forbidden_slices.rs:74:25
+    |                         ---------- inside `R8` at $DIR/forbidden_slices.rs:75:25
221 error[E0080]: could not evaluate static initializer
222   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


232 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
233    |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:79:34
+   ::: $DIR/forbidden_slices.rs:80:34
236    |
236    |
237 LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
-    |                                  ----------------------------------------------- inside `R9` at $DIR/forbidden_slices.rs:79:34
+    |                                  ----------------------------------------------- inside `R9` at $DIR/forbidden_slices.rs:80:34
240 error[E0080]: could not evaluate static initializer
241   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


251 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
252    |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-   ::: $DIR/forbidden_slices.rs:80:35
+   ::: $DIR/forbidden_slices.rs:81:35
255    |
255    |
256 LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };
-    |                                   ------------------------ inside `R10` at $DIR/forbidden_slices.rs:80:35
+    |                                   ------------------------ inside `R10` at $DIR/forbidden_slices.rs:81:35
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
   |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |         inside `std::slice::from_raw_parts::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:97:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:34
   |
   |
LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                  ------------------------------ inside `S0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |         inside `std::slice::from_raw_parts::<'_, ()>` at /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:20:33
   |
LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };
   |                                 ------------------------------ inside `S1` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:20:33
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc18 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:97:9
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
  --> /checkout/library/core/src/slice/raw.rs:97:9
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
   |         |
   |         |
   |         dereferencing pointer failed: alloc61 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |         inside `std::slice::from_raw_parts::<'_, u64>` at /checkout/library/core/src/slice/raw.rs:97:9
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:44:5
   |
LL |     from_raw_parts(ptr, 1)
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:44:5
   |     ---------------------- inside `S8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:44:5

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:770:18
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:770:18
   |
   |
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:34
   |
   |
LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   |                                  ---------------------------------------- inside `R0` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:768:9
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
   |         |
   |         |
   |         the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', /checkout/library/core/src/ptr/const_ptr.rs:768:9
   |         inside `ptr::const_ptr::<impl *const ()>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:768:9
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, ()>` at /checkout/library/core/src/slice/raw.rs:219:42
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
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add` at /checkout/library/core/src/ptr/const_ptr.rs:894:18
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
   = note: the raw bytes of the constant (size: 16, align: 8) {
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
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:63:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:63:1
   |
LL | pub static R6: &[bool] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:68:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:68:1
   |
LL | pub static R7: &[u16] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾────alloc134+0x1─────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

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
   |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add` at /checkout/library/core/src/ptr/const_ptr.rs:894:18
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:75:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(1))
   |                         ---------- inside `R8` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:75:25
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:770:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  `ptr_offset_from_unsigned` called on pointers into different allocations
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:770:18
   |
   |
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:80:34
   |
   |
LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
   |                                  ----------------------------------------------- inside `R9` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:80:34
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:770:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  |
   |                  `ptr_offset_from_unsigned` called on pointers into different allocations
   |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr` at /checkout/library/core/src/ptr/const_ptr.rs:770:18
   |
   |
  ::: /checkout/library/core/src/slice/raw.rs:219:42
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
   |                                          ------------------------------ inside `from_ptr_range::<'_, u32>` at /checkout/library/core/src/slice/raw.rs:219:42
  ::: /checkout/src/test/ui/const-ptr/forbidden_slices.rs:81:35
   |
   |
LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };
   |                                   ------------------------ inside `R10` at /checkout/src/test/ui/const-ptr/forbidden_slices.rs:81:35
error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---
1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:23:1
+   --> $DIR/ub-enum.rs:24:1
3    |
4 LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
5    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000001, but expected a valid enum tag
10            }
11 
12 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:26:1
-   --> $DIR/ub-enum.rs:26:1
+   --> $DIR/ub-enum.rs:27:1
14    |
15 LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
16    | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

22    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
24 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:30:1
+   --> $DIR/ub-enum.rs:31:1
26    |
26    |
27 LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
28    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

33    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
35 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:43:1
+   --> $DIR/ub-enum.rs:44:1
37    |
37    |
38 LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
39    | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000000, but expected a valid enum tag
44            }
45 
46 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:45:1
-   --> $DIR/ub-enum.rs:45:1
+   --> $DIR/ub-enum.rs:46:1
48    |
49 LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
50    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

55    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
57 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:49:1
+   --> $DIR/ub-enum.rs:50:1
59    |
59    |
60 LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
61    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

66    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
68 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-enum.rs:59:42
+   --> $DIR/ub-enum.rs:60:42
70    |
70    |
71 LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
72    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
73 
74 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:64:1
+   --> $DIR/ub-enum.rs:65:1
+   --> $DIR/ub-enum.rs:65:1
76    |
77 LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
78    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

83    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
85 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:82:1
+   --> $DIR/ub-enum.rs:83:1
87    |
87    |
88 LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
89    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(B)>.0: encountered a value of the never type `!`
94            }
95 
96 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:84:1
-   --> $DIR/ub-enum.rs:84:1
+   --> $DIR/ub-enum.rs:85:1
98    |
99 LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
100    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
105            }
106 
107 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:92:1
-   --> $DIR/ub-enum.rs:92:1
+   --> $DIR/ub-enum.rs:93:1
109    |
110 LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
111    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Some)>.0.1: encountered 0xffffffff, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
116            }
117 
118 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-enum.rs:97:77
-   --> $DIR/ub-enum.rs:97:77
+   --> $DIR/ub-enum.rs:98:77
120    |
121 LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
122    |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
123 
124 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-enum.rs:99:77
+   --> $DIR/ub-enum.rs:100:77
+   --> $DIR/ub-enum.rs:100:77
126    |
127 LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
128    |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
132 For more information about this error, try `rustc --explain E0080`.
133 Future incompatibility report: Future breakage diagnostic:
134 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:26:1
-   --> $DIR/ub-enum.rs:26:1
+   --> $DIR/ub-enum.rs:27:1
136    |
137 LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
138    | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
145 
146 Future breakage diagnostic:
147 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:30:1
-   --> $DIR/ub-enum.rs:30:1
+   --> $DIR/ub-enum.rs:31:1
149    |
150 LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
151    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
158 
159 Future breakage diagnostic:
160 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:45:1
-   --> $DIR/ub-enum.rs:45:1
+   --> $DIR/ub-enum.rs:46:1
162    |
163 LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
164    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
171 
172 Future breakage diagnostic:
173 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:49:1
-   --> $DIR/ub-enum.rs:49:1
+   --> $DIR/ub-enum.rs:50:1
175    |
176 LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
177    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
184 
185 Future breakage diagnostic:
186 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:64:1
-   --> $DIR/ub-enum.rs:64:1
+   --> $DIR/ub-enum.rs:65:1
188    |
189 LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
190    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

The actual 64bit.stderr differed from the expected 64bit.stderr.
The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/ub-enum.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000001, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:27:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:27:1
   |
LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:31:1
   |
   |
LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:44:1
   |
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000000, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:46:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:46:1
   |
LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:50:1
   |
   |
LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:60:42
   |
   |
LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:65:1
   |
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:83:1
   |
   |
LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(B)>.0: encountered a value of the never type `!`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:85:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:85:1
   |
LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:93:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:93:1
   |
LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Some)>.0.1: encountered 0xffffffff, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               78 00 00 00 ff ff ff ff                         │ x.......

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:98:77
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
   |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:100:77
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
   |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:27:1
   |
LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:31:1
   |
   |
LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:46:1
   |
   |
LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:50:1
   |
   |
LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:65:1
   |
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported


---- [ui] src/test/ui/consts/const-eval/ub-ref-ptr.rs stdout ----
diff of 64bit.stderr:
diff of 64bit.stderr:

1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:13:1
+   --> $DIR/ub-ref-ptr.rs:14:1
3    |
4 LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
5    | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
10            }
11 
12 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:17:1
-   --> $DIR/ub-ref-ptr.rs:17:1
+   --> $DIR/ub-ref-ptr.rs:18:1
14    |
15 LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
16    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned box (required 2 byte alignment but found 1)
21            }
22 
23 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:21:1
-   --> $DIR/ub-ref-ptr.rs:21:1
+   --> $DIR/ub-ref-ptr.rs:22:1
25    |
26 LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
27    | ^^^^^^^^^^^^^^^^ constructing invalid value: encountered a null reference
32            }
33 
34 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:24:1
-   --> $DIR/ub-ref-ptr.rs:24:1
+   --> $DIR/ub-ref-ptr.rs:25:1
36    |
37 LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
38    | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a null box
43            }
44 
45 error: any use of this value will cause an error
-   --> $DIR/ub-ref-ptr.rs:31:1
-   --> $DIR/ub-ref-ptr.rs:31:1
+   --> $DIR/ub-ref-ptr.rs:32:1
47    |
48 LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
49    | ^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

55    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
57 error: any use of this value will cause an error
-   --> $DIR/ub-ref-ptr.rs:35:39
+   --> $DIR/ub-ref-ptr.rs:36:39
59    |
59    |
60 LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
61    | ----------------------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

66    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
68 error: any use of this value will cause an error
-   --> $DIR/ub-ref-ptr.rs:35:38
+   --> $DIR/ub-ref-ptr.rs:36:38
70    |
70    |
71 LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];

75    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
76 
77 error: any use of this value will cause an error
77 error: any use of this value will cause an error
-   --> $DIR/ub-ref-ptr.rs:41:86
+   --> $DIR/ub-ref-ptr.rs:42:86
79    |
80 LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
81    | ------------------------------------------                                           ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

86    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
88 error: any use of this value will cause an error
-   --> $DIR/ub-ref-ptr.rs:41:85
+   --> $DIR/ub-ref-ptr.rs:42:85
90    |
90    |
91 LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };

95    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
96 
97 error[E0080]: it is undefined behavior to use this value
97 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:47:1
+   --> $DIR/ub-ref-ptr.rs:48:1
99    |
100 LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
101    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (address 0x539 is unallocated)
106            }
107 
108 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:50:1
-   --> $DIR/ub-ref-ptr.rs:50:1
+   --> $DIR/ub-ref-ptr.rs:51:1
110    |
111 LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
112    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling box (address 0x539 is unallocated)
117            }
118 
119 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-ref-ptr.rs:53:41
-   --> $DIR/ub-ref-ptr.rs:53:41
+   --> $DIR/ub-ref-ptr.rs:54:41
121    |
122 LL | const UNINIT_PTR: *const i32 = unsafe { MaybeUninit { uninit: () }.init };
123    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
124 
125 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:57:1
+   --> $DIR/ub-ref-ptr.rs:58:1
+   --> $DIR/ub-ref-ptr.rs:58:1
127    |
128 LL | const NULL_FN_PTR: fn() = unsafe { mem::transmute(0usize) };
129    | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered null pointer, but expected a function pointer
134            }
135 
136 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-ref-ptr.rs:59:38
-   --> $DIR/ub-ref-ptr.rs:59:38
+   --> $DIR/ub-ref-ptr.rs:60:38
138    |
139 LL | const UNINIT_FN_PTR: fn() = unsafe { MaybeUninit { uninit: () }.init };
140    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
141 
142 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-ref-ptr.rs:62:1
+   --> $DIR/ub-ref-ptr.rs:63:1
+   --> $DIR/ub-ref-ptr.rs:63:1
144    |
---
1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-uninhabit.rs:15:1
+   --> $DIR/ub-uninhabit.rs:16:1
3    |
4 LL | const BAD_BAD_BAD: Bar = unsafe { MaybeUninit { uninit: () }.init };
5    | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a value of uninhabited type Bar

8    = note: the raw bytes of the constant (size: 0, align: 1) {}
10 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-uninhabit.rs:18:1
+   --> $DIR/ub-uninhabit.rs:19:1
12    |
12    |
13 LL | const BAD_BAD_REF: &Bar = unsafe { mem::transmute(1usize) };
14    | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to uninhabited type Bar
19            }
20 
21 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-uninhabit.rs:21:1
-   --> $DIR/ub-uninhabit.rs:21:1
+   --> $DIR/ub-uninhabit.rs:22:1
23    |
24 LL | const BAD_BAD_ARRAY: [Bar; 1] = unsafe { MaybeUninit { uninit: () }.init };
25    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at [0]: encountered a value of uninhabited type Bar

The actual 64bit.stderr differed from the expected 64bit.stderr.
The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit/ub-uninhabit.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-uninhabit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const BAD_BAD_BAD: Bar = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a value of uninhabited type Bar
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 0, align: 1) {}
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs:19:1
   |
   |
LL | const BAD_BAD_REF: &Bar = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to uninhabited type Bar
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs:22:1
  --> /checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs:22:1
   |
LL | const BAD_BAD_ARRAY: [Bar; 1] = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at [0]: encountered a value of uninhabited type Bar
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 0, align: 1) {}
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---
1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-nonnull.rs:12:1
+   --> $DIR/ub-nonnull.rs:13:1
3    |
4 LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
10            }
11 
12 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-nonnull.rs:19:30
-   --> $DIR/ub-nonnull.rs:19:30
+   --> $DIR/ub-nonnull.rs:20:30
14    |
15 LL |     let out_of_bounds_ptr = &ptr[255];
16    |                              ^^^^^^^^ dereferencing pointer failed: alloc11 has size 1, so pointer to 256 bytes starting at offset 0 is out-of-bounds
17 
18 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-nonnull.rs:23:1
+   --> $DIR/ub-nonnull.rs:24:1
+   --> $DIR/ub-nonnull.rs:24:1
20    |
21 LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
22    | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
27            }
28 
29 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-nonnull.rs:25:1
-   --> $DIR/ub-nonnull.rs:25:1
+   --> $DIR/ub-nonnull.rs:26:1
31    |
32 LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
33    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
38            }
39 
40 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-nonnull.rs:33:36
-   --> $DIR/ub-nonnull.rs:33:36
+   --> $DIR/ub-nonnull.rs:34:36
42    |
43 LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
44    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
45 
46 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-nonnull.rs:42:1
+   --> $DIR/ub-nonnull.rs:43:1
+   --> $DIR/ub-nonnull.rs:43:1
48    |
49 LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
50    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 42, but expected something in the range 10..=30
55            }
56 
57 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-nonnull.rs:48:1
-   --> $DIR/ub-nonnull.rs:48:1
+   --> $DIR/ub-nonnull.rs:49:1
59    |
60 LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
61    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 20, but expected something less or equal to 10, or greater or equal to 30

The actual 64bit.stderr differed from the expected 64bit.stderr.
The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.64bit.stderr
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
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:20:30
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:20:30
   |
LL |     let out_of_bounds_ptr = &ptr[255]; //~ ERROR evaluation of constant value failed
   |                              ^^^^^^^^ dereferencing pointer failed: alloc11 has size 1, so pointer to 256 bytes starting at offset 0 is out-of-bounds
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:24:1
   |
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:26:1
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:26:1
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:34:36
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:34:36
   |
LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:43:1
   |
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 42, but expected something in the range 10..=30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               2a 00 00 00                                     │ *...

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:49:1
   |
   |
LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 20, but expected something less or equal to 10, or greater or equal to 30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: aborting due to 7 previous errors

---
1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:37:1
+   --> $DIR/ub-wide-ptr.rs:38:1
3    |
4 LL | const STR_TOO_LONG: &str = unsafe { mem::transmute((&42u8, 999usize)) };
5    | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
10            }
11 
12 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:39:1
-   --> $DIR/ub-wide-ptr.rs:39:1
+   --> $DIR/ub-wide-ptr.rs:40:1
14    |
15 LL | const NESTED_STR_MUCH_TOO_LONG: (&str,) = (unsafe { mem::transmute((&42, usize::MAX)) },);
16    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered invalid reference metadata: slice is bigger than largest supported object
21            }
22 
23 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:42:1
-   --> $DIR/ub-wide-ptr.rs:42:1
+   --> $DIR/ub-wide-ptr.rs:43:1
25    |
26 LL | const STR_LENGTH_PTR: &str = unsafe { mem::transmute((&42u8, &3)) };
27    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

33    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
35 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:46:1
+   --> $DIR/ub-wide-ptr.rs:47:1
37    |
37    |
38 LL | const MY_STR_LENGTH_PTR: &MyStr = unsafe { mem::transmute((&42u8, &3)) };
39    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

44    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
46 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:49:1
+   --> $DIR/ub-wide-ptr.rs:50:1
48    |
48    |
49 LL | const MY_STR_MUCH_TOO_LONG: &MyStr = unsafe { mem::transmute((&42u8, usize::MAX)) };
50    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered invalid reference metadata: slice is bigger than largest supported object
55            }
56 
57 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:53:1
-   --> $DIR/ub-wide-ptr.rs:53:1
+   --> $DIR/ub-wide-ptr.rs:54:1
59    |
60 LL | const STR_NO_INIT: &str = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
61    | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>: encountered uninitialized data in `str`
66            }
67 
68 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:56:1
-   --> $DIR/ub-wide-ptr.rs:56:1
+   --> $DIR/ub-wide-ptr.rs:57:1
70    |
71 LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
72    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.0: encountered uninitialized data in `str`
77            }
78 
79 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:63:1
-   --> $DIR/ub-wide-ptr.rs:63:1
+   --> $DIR/ub-wide-ptr.rs:64:1
81    |
82 LL | const SLICE_LENGTH_UNINIT: &[u8] = unsafe {
83    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
84 
85 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:70:1
+   --> $DIR/ub-wide-ptr.rs:71:1
+   --> $DIR/ub-wide-ptr.rs:71:1
87    |
88 LL | const SLICE_TOO_LONG: &[u8] = unsafe { mem::transmute((&42u8, 999usize)) };
89    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
94            }
95 
96 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:73:1
-   --> $DIR/ub-wide-ptr.rs:73:1
+   --> $DIR/ub-wide-ptr.rs:74:1
98    |
99 LL | const SLICE_TOO_LONG_OVERFLOW: &[u32] = unsafe { mem::transmute((&42u32, isize::MAX)) };
100    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered invalid reference metadata: slice is bigger than largest supported object
105            }
106 
107 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:76:1
-   --> $DIR/ub-wide-ptr.rs:76:1
+   --> $DIR/ub-wide-ptr.rs:77:1
109    |
110 LL | const SLICE_LENGTH_PTR: &[u8] = unsafe { mem::transmute((&42u8, &3)) };
111    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

116    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
118 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:80:1
+   --> $DIR/ub-wide-ptr.rs:81:1
120    |
120    |
121 LL | const SLICE_TOO_LONG_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, 999usize)) };
122    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling box (going beyond the bounds of its allocation)
127            }
128 
129 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:83:1
-   --> $DIR/ub-wide-ptr.rs:83:1
+   --> $DIR/ub-wide-ptr.rs:84:1
131    |
132 LL | const SLICE_LENGTH_PTR_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, &3)) };
133    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

138    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
140 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:88:1
+   --> $DIR/ub-wide-ptr.rs:89:1
142    |
142    |
143 LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];
144    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x03, but expected a boolean
149            }
150 
151 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:88:40
-   --> $DIR/ub-wide-ptr.rs:88:40
+   --> $DIR/ub-wide-ptr.rs:89:40
153    |
154 LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];

158    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
159 
160 error[E0080]: it is undefined behavior to use this value
160 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:96:1
+   --> $DIR/ub-wide-ptr.rs:97:1
162    |
163 LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);
164    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.0: encountered 0x03, but expected a boolean
169            }
170 
171 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:96:42
-   --> $DIR/ub-wide-ptr.rs:96:42
+   --> $DIR/ub-wide-ptr.rs:97:42
173    |
174 LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);

178    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
179 
180 error[E0080]: it is undefined behavior to use this value
180 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:101:1
+   --> $DIR/ub-wide-ptr.rs:102:1
182    |
183 LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);
184    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.1[0]: encountered 0x03, but expected a boolean
189            }
190 
191 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:101:42
-   --> $DIR/ub-wide-ptr.rs:101:42
+   --> $DIR/ub-wide-ptr.rs:102:42
193    |
194 LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);

198    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
199 
200 error[E0080]: evaluation of constant value failed
200 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:110:1
+   --> $DIR/ub-wide-ptr.rs:111:1
202    |
203 LL | const RAW_SLICE_LENGTH_UNINIT: *const [u8] = unsafe {
204    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
205 
206 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:119:1
+   --> $DIR/ub-wide-ptr.rs:120:1
+   --> $DIR/ub-wide-ptr.rs:120:1
208    |
209 LL | const TRAIT_OBJ_SHORT_VTABLE_1: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u8))) };
210    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered allocN, but expected a vtable pointer
215            }
216 
217 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:123:1
-   --> $DIR/ub-wide-ptr.rs:123:1
+   --> $DIR/ub-wide-ptr.rs:124:1
219    |
220 LL | const TRAIT_OBJ_SHORT_VTABLE_2: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u64))) };
221    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered allocN, but expected a vtable pointer
226            }
227 
228 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:127:1
-   --> $DIR/ub-wide-ptr.rs:127:1
+   --> $DIR/ub-wide-ptr.rs:128:1
230    |
231 LL | const TRAIT_OBJ_INT_VTABLE: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, 4usize))) };
232    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered 0x4[noalloc], but expected a vtable pointer
237            }
238 
239 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:130:57
-   --> $DIR/ub-wide-ptr.rs:130:57
+   --> $DIR/ub-wide-ptr.rs:131:57
241    |
242 LL | const TRAIT_OBJ_UNALIGNED_VTABLE: &dyn Trait = unsafe { mem::transmute((&92u8, &[0u8; 128])) };
243    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using allocN as vtable pointer but it does not point to a vtable
244 
245 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:133:57
+   --> $DIR/ub-wide-ptr.rs:134:57
+   --> $DIR/ub-wide-ptr.rs:134:57
247    |
248 LL | const TRAIT_OBJ_BAD_DROP_FN_NULL: &dyn Trait = unsafe { mem::transmute((&92u8, &[0usize; 8])) };
249    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using allocN as vtable pointer but it does not point to a vtable
250 
251 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:136:56
+   --> $DIR/ub-wide-ptr.rs:137:56
+   --> $DIR/ub-wide-ptr.rs:137:56
253    |
254 LL | const TRAIT_OBJ_BAD_DROP_FN_INT: &dyn Trait = unsafe { mem::transmute((&92u8, &[1usize; 8])) };
255    |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using allocN as vtable pointer but it does not point to a vtable
256 
257 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:139:1
+   --> $DIR/ub-wide-ptr.rs:140:1
+   --> $DIR/ub-wide-ptr.rs:140:1
259    |
260 LL | const TRAIT_OBJ_BAD_DROP_FN_NOT_FN_PTR: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &[&42u8; 8]))) };
261    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered allocN, but expected a vtable pointer
266            }
267 
268 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:144:1
-   --> $DIR/ub-wide-ptr.rs:144:1
+   --> $DIR/ub-wide-ptr.rs:145:1
270    |
271 LL | const TRAIT_OBJ_CONTENT_INVALID: &dyn Trait = unsafe { mem::transmute::<_, &bool>(&3u8) };
272    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.<dyn-downcast>: encountered 0x03, but expected a boolean
277            }
278 
279 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:149:1
-   --> $DIR/ub-wide-ptr.rs:149:1
+   --> $DIR/ub-wide-ptr.rs:150:1
281    |
282 LL | const RAW_TRAIT_OBJ_VTABLE_NULL: *const dyn Trait = unsafe { mem::transmute((&92u8, 0usize)) };
283    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered null pointer, but expected a vtable pointer
288            }
289 
290 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:151:1
-   --> $DIR/ub-wide-ptr.rs:151:1
+   --> $DIR/ub-wide-ptr.rs:152:1
292    |
293 LL | const RAW_TRAIT_OBJ_VTABLE_INVALID: *const dyn Trait = unsafe { mem::transmute((&92u8, &3u64)) };
294    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered allocN, but expected a vtable pointer
299            }
300 
301 error[E0080]: could not evaluate static initializer
-   --> $DIR/ub-wide-ptr.rs:157:5
-   --> $DIR/ub-wide-ptr.rs:157:5
+   --> $DIR/ub-wide-ptr.rs:158:5
303    |
304 LL |     mem::transmute::<_, &dyn Trait>((&92u8, 0usize))
305    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer use: null pointer is a dangling pointer (it has no provenance)
306 
307 error[E0080]: could not evaluate static initializer
-   --> $DIR/ub-wide-ptr.rs:161:5
+   --> $DIR/ub-wide-ptr.rs:162:5
+   --> $DIR/ub-wide-ptr.rs:162:5
309    |
310 LL |     mem::transmute::<_, &dyn Trait>((&92u8, &3u64))
311    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using allocN as vtable pointer but it does not point to a vtable
315 For more information about this error, try `rustc --explain E0080`.
316 Future incompatibility report: Future breakage diagnostic:
317 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:42:1
-   --> $DIR/ub-wide-ptr.rs:42:1
+   --> $DIR/ub-wide-ptr.rs:43:1
319    |
320 LL | const STR_LENGTH_PTR: &str = unsafe { mem::transmute((&42u8, &3)) };
321    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
328 
329 Future breakage diagnostic:
330 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:46:1
-   --> $DIR/ub-wide-ptr.rs:46:1
+   --> $DIR/ub-wide-ptr.rs:47:1
332    |
333 LL | const MY_STR_LENGTH_PTR: &MyStr = unsafe { mem::transmute((&42u8, &3)) };
334    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
341 
342 Future breakage diagnostic:
343 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:76:1
-   --> $DIR/ub-wide-ptr.rs:76:1
+   --> $DIR/ub-wide-ptr.rs:77:1
345    |
346 LL | const SLICE_LENGTH_PTR: &[u8] = unsafe { mem::transmute((&42u8, &3)) };
347    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
354 
355 Future breakage diagnostic:
356 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:83:1
-   --> $DIR/ub-wide-ptr.rs:83:1
+   --> $DIR/ub-wide-ptr.rs:84:1
358    |
359 LL | const SLICE_LENGTH_PTR_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, &3)) };
360    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
367 
368 Future breakage diagnostic:
369 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:88:40
-   --> $DIR/ub-wide-ptr.rs:88:40
+   --> $DIR/ub-wide-ptr.rs:89:40
371    |
372 LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];

378 
379 Future breakage diagnostic:
380 error: any use of this value will cause an error
380 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:96:42
+   --> $DIR/ub-wide-ptr.rs:97:42
382    |
383 LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);

389 
390 Future breakage diagnostic:
391 error: any use of this value will cause an error
391 error: any use of this value will cause an error
-   --> $DIR/ub-wide-ptr.rs:101:42
+   --> $DIR/ub-wide-ptr.rs:102:42
393    |
394 LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);


The actual 64bit.stderr differed from the expected 64bit.stderr.
The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/ub-wide-ptr.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-wide-ptr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const STR_TOO_LONG: &str = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc8────────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:40:1
   |
   |
LL | const NESTED_STR_MUCH_TOO_LONG: (&str,) = (unsafe { mem::transmute((&42, usize::MAX)) },);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc14───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:43:1
   |
   |
LL | const STR_LENGTH_PTR: &str = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:47:1
   |
   |
LL | const MY_STR_LENGTH_PTR: &MyStr = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:50:1
   |
   |
LL | const MY_STR_MUCH_TOO_LONG: &MyStr = unsafe { mem::transmute((&42u8, usize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc34───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:54:1
   |
   |
LL | const STR_NO_INIT: &str = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:57:1
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:57:1
   |
LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.0: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:64:1
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:64:1
   |
LL | const SLICE_LENGTH_UNINIT: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:71:1
   |
   |
LL | const SLICE_TOO_LONG: &[u8] = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc59───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:74:1
   |
   |
LL | const SLICE_TOO_LONG_OVERFLOW: &[u32] = unsafe { mem::transmute((&42u32, isize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc67───────╼ ff ff ff ff ff ff ff 7f │ ╾──────╼........

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:77:1
   |
   |
LL | const SLICE_LENGTH_PTR: &[u8] = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:81:1
   |
   |
LL | const SLICE_TOO_LONG_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling box (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc79───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:84:1
   |
   |
LL | const SLICE_LENGTH_PTR_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:89:1
   |
   |
LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
---
1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/issue-83182.rs:5:1
+   --> $DIR/issue-83182.rs:6:1
3    |
4 LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[&()]) };
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

The actual 64bit.stderr differed from the expected 64bit.stderr.
The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-83182/issue-83182.64bit.stderr
To only update this specific test, also pass `--test-args consts/issue-83182.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-83182.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-83182" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-83182/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[&()]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error: aborting due to previous error

---
1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/validate_never_arrays.rs:4:1
+   --> $DIR/validate_never_arrays.rs:5:1
3    |
4 LL | const _: &[!; 1] = unsafe { &*(1_usize as *const [!; 1]) };
5    | ^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to uninhabited type [!; 1]
10            }
11 
12 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/validate_never_arrays.rs:7:1
-   --> $DIR/validate_never_arrays.rs:7:1
+   --> $DIR/validate_never_arrays.rs:8:1
14    |
15 LL | const _: &[!] = unsafe { &*(1_usize as *const [!; 1]) };
16    | ^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered a value of the never type `!`
21            }
22 
23 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/validate_never_arrays.rs:8:1
-   --> $DIR/validate_never_arrays.rs:8:1
+   --> $DIR/validate_never_arrays.rs:9:1
25    |
26 LL | const _: &[!] = unsafe { &*(1_usize as *const [!; 42]) };
27    | ^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered a value of the never type `!`

The actual 64bit.stderr differed from the expected 64bit.stderr.
The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/validate_never_arrays/validate_never_arrays.64bit.stderr
To only update this specific test, also pass `--test-args consts/validate_never_arrays.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/validate_never_arrays.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/validate_never_arrays" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/validate_never_arrays/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const _: &[!; 1] = unsafe { &*(1_usize as *const [!; 1]) }; //~ ERROR undefined behavior
   | ^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to uninhabited type [!; 1]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/validate_never_arrays.rs:8:1
  --> /checkout/src/test/ui/consts/validate_never_arrays.rs:8:1
   |
LL | const _: &[!] = unsafe { &*(1_usize as *const [!; 1]) }; //~ ERROR undefined behavior
   | ^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered a value of the never type `!`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/validate_never_arrays.rs:9:1
  --> /checkout/src/test/ui/consts/validate_never_arrays.rs:9:1
   |
LL | const _: &[!] = unsafe { &*(1_usize as *const [!; 42]) }; //~ ERROR undefined behavior
   | ^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered a value of the never type `!`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               01 00 00 00 00 00 00 00 2a 00 00 00 00 00 00 00 │ ........*.......

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
