plain
diff of 32bit.stderr:

2   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
3    |
4 LL |         &*ptr::slice_from_raw_parts(data, len)
-    |         |
-    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
-    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
-    |         inside `std::slice::from_raw_parts::<'_, u32>`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
-   ::: $DIR/forbidden_slices.rs:18:34
-   ::: $DIR/forbidden_slices.rs:18:34
+ note: inside `std::slice::from_raw_parts::<'_, u32>`
+   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
11    |
+ LL |         &*ptr::slice_from_raw_parts(data, len)
+ note: inside `S0`
+   --> $DIR/forbidden_slices.rs:18:34
+    |
+    |
12 LL | pub static S0: &[u32] = unsafe { from_raw_parts(ptr::null(), 0) };
-    |                                  ------------------------------ inside `S0`
14 
15 error[E0080]: could not evaluate static initializer
16   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL


17    |
18 LL |         &*ptr::slice_from_raw_parts(data, len)
-    |         |
-    |         dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
-    |         inside `std::slice::from_raw_parts::<'_, ()>`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
23    |
-   ::: $DIR/forbidden_slices.rs:19:33
+ note: inside `std::slice::from_raw_parts::<'_, ()>`
+   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
25    |
+ LL |         &*ptr::slice_from_raw_parts(data, len)
+ note: inside `S1`
+   --> $DIR/forbidden_slices.rs:19:33
+    |
+    |
26 LL | pub static S1: &[()] = unsafe { from_raw_parts(ptr::null(), 0) };
-    |                                 ------------------------------ inside `S1`
28 
29 error[E0080]: could not evaluate static initializer
30   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL


31    |
32 LL |         &*ptr::slice_from_raw_parts(data, len)
-    |         |
-    |         |
-    |         dereferencing pointer failed: ALLOC_ID has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
-    |         inside `std::slice::from_raw_parts::<'_, u32>`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: ALLOC_ID has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
-   ::: $DIR/forbidden_slices.rs:22:34
-   ::: $DIR/forbidden_slices.rs:22:34
+ note: inside `std::slice::from_raw_parts::<'_, u32>`
+   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
39    |
+ LL |         &*ptr::slice_from_raw_parts(data, len)
+ note: inside `S2`
+   --> $DIR/forbidden_slices.rs:22:34
+    |
+    |
40 LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };
-    |                                  ---------------------- inside `S2`
42 
43 error[E0080]: it is undefined behavior to use this value
44   --> $DIR/forbidden_slices.rs:25:1


89   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
90    |
91 LL |         &*ptr::slice_from_raw_parts(data, len)
-    |         |
-    |         |
-    |         dereferencing pointer failed: ALLOC_ID has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
-    |         inside `std::slice::from_raw_parts::<'_, u64>`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: ALLOC_ID has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
-   ::: $DIR/forbidden_slices.rs:43:5
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
+ note: inside `std::slice::from_raw_parts::<'_, u64>`
+   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
98    |
+ LL |         &*ptr::slice_from_raw_parts(data, len)
+ note: inside `S8`
+   --> $DIR/forbidden_slices.rs:43:5
+    |
99 LL |     from_raw_parts(ptr, 1)
99 LL |     from_raw_parts(ptr, 1)
-    |     ---------------------- inside `S8`
+    |     ^^^^^^^^^^^^^^^^^^^^^^
101 
102 error[E0080]: could not evaluate static initializer
103   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

104    |
105 LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
-    |                  |
-    |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
-    |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
110    |
-   ::: $SRC_DIR/core/src/slice/raw.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
112    |
+ LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: inside `from_ptr_range::<'_, u32>`
+   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
+    |
113 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
-    |                                          ------------------------------ inside `from_ptr_range::<'_, u32>`
+ note: inside `R0`
+   --> $DIR/forbidden_slices.rs:46:34
115    |
-   ::: $DIR/forbidden_slices.rs:46:34
-   ::: $DIR/forbidden_slices.rs:46:34
-    |
118 LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
-    |                                  ---------------------------------------- inside `R0`
120 
121 error[E0080]: could not evaluate static initializer
122   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


123    |
124 LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
-    |         |
-    |         |
-    |         the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |         inside `ptr::const_ptr::<impl *const ()>::sub_ptr`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $SRC_DIR/core/src/slice/raw.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const ()>::sub_ptr`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
131    |
131    |
+ LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
+ note: inside `from_ptr_range::<'_, ()>`
+   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
+    |
+    |
132 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
-    |                                          ------------------------------ inside `from_ptr_range::<'_, ()>`
+ note: inside `R1`
+   --> $DIR/forbidden_slices.rs:47:33
134    |
-   ::: $DIR/forbidden_slices.rs:47:33
-   ::: $DIR/forbidden_slices.rs:47:33
-    |
137 LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
-    |                                 ---------------------------------------- inside `R1`
+    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
140    = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)
141 
142 error[E0080]: could not evaluate static initializer
142 error[E0080]: could not evaluate static initializer

143   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
144    |
145 LL |         unsafe { intrinsics::offset(self, count) }
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: ALLOC_ID has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
+ note: inside `ptr::const_ptr::<impl *const u32>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+ LL |         unsafe { intrinsics::offset(self, count) }
146    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |                  |
-    |                  out-of-bounds pointer arithmetic: ALLOC_ID has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
-    |                  inside `ptr::const_ptr::<impl *const u32>::offset`
+ note: inside `ptr::const_ptr::<impl *const u32>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+    |
151 LL |         unsafe { self.offset(count as isize) }
-    |                  --------------------------- inside `ptr::const_ptr::<impl *const u32>::add`
+ note: inside `R2`
+   --> $DIR/forbidden_slices.rs:50:25
153    |
-   ::: $DIR/forbidden_slices.rs:50:25
-   ::: $DIR/forbidden_slices.rs:50:25
-    |
156 LL |     from_ptr_range(ptr..ptr.add(2))
-    |                         ---------- inside `R2`
158 
159 error[E0080]: it is undefined behavior to use this value
160   --> $DIR/forbidden_slices.rs:52:1


205   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
206    |
207 LL |         unsafe { intrinsics::offset(self, count) }
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: ALLOC_ID has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
+ note: inside `ptr::const_ptr::<impl *const u64>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+ LL |         unsafe { intrinsics::offset(self, count) }
208    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |                  |
-    |                  out-of-bounds pointer arithmetic: ALLOC_ID has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
-    |                  inside `ptr::const_ptr::<impl *const u64>::offset`
+ note: inside `ptr::const_ptr::<impl *const u64>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+    |
213 LL |         unsafe { self.offset(count as isize) }
-    |                  --------------------------- inside `ptr::const_ptr::<impl *const u64>::add`
+ note: inside `R8`
+   --> $DIR/forbidden_slices.rs:74:25
215    |
-   ::: $DIR/forbidden_slices.rs:74:25
-   ::: $DIR/forbidden_slices.rs:74:25
-    |
218 LL |     from_ptr_range(ptr..ptr.add(1))
-    |                         ---------- inside `R8`
220 
221 error[E0080]: could not evaluate static initializer
222   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


223    |
224 LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
-    |                  |
-    |                  `ptr_offset_from_unsigned` called on pointers into different allocations
-    |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called on pointers into different allocations
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called on pointers into different allocations
229    |
-   ::: $SRC_DIR/core/src/slice/raw.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
231    |
+ LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: inside `from_ptr_range::<'_, u32>`
+   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
+    |
232 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
-    |                                          ------------------------------ inside `from_ptr_range::<'_, u32>`
+ note: inside `R9`
+   --> $DIR/forbidden_slices.rs:79:34
234    |
-   ::: $DIR/forbidden_slices.rs:79:34
-   ::: $DIR/forbidden_slices.rs:79:34
-    |
237 LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
-    |                                  ----------------------------------------------- inside `R9`
239 
240 error[E0080]: could not evaluate static initializer
241   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


242    |
243 LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
-    |                  |
-    |                  `ptr_offset_from_unsigned` called on pointers into different allocations
-    |                  inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called on pointers into different allocations
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called on pointers into different allocations
248    |
-   ::: $SRC_DIR/core/src/slice/raw.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
250    |
+ LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: inside `from_ptr_range::<'_, u32>`
+   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
+    |
251 LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
-    |                                          ------------------------------ inside `from_ptr_range::<'_, u32>`
+ note: inside `R10`
+   --> $DIR/forbidden_slices.rs:80:35
253    |
-   ::: $DIR/forbidden_slices.rs:80:35
-   ::: $DIR/forbidden_slices.rs:80:35
-    |
256 LL | pub static R10: &[u32] = unsafe { from_ptr_range(&D0..&D0) };
-    |                                   ------------------------ inside `R10`
258 
259 error: aborting due to 18 previous errors
260 



The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/forbidden_slices/forbidden_slices.32bit.stderr
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
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   |
note: inside `std::slice::from_raw_parts::<'_, u32>`
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
note: inside `S0`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:18:34
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
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:19:33
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
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:22:34
   |
   |
LL | pub static S2: &[u32] = unsafe { from_raw_parts(&D0, 2) };

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:25:1
   |
   |
LL | pub static S4: &[u8] = unsafe { from_raw_parts((&D1) as *const _ as _, 1) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
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
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:29:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:29:1
   |
LL | pub static S6: &[bool] = unsafe { from_raw_parts((&D0) as *const _ as _, 4) }; //~ ERROR: it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:32:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:32:1
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
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: alloc61 has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
   |
note: inside `std::slice::from_raw_parts::<'_, u64>`
   |
   |
LL |         &*ptr::slice_from_raw_parts(data, len)
note: inside `S8`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:43:5
   |
LL |     from_raw_parts(ptr, 1)
LL |     from_raw_parts(ptr, 1)
   |     ^^^^^^^^^^^^^^^^^^^^^^

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:806:18
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
  --> /checkout/library/core/src/ptr/const_ptr.rs:806:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from_unsigned(self, origin) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `from_ptr_range::<'_, u32>`
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
note: inside `R0`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:46:34
   |
   |
LL | pub static R0: &[u32] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:804:9
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize', /checkout/library/core/src/ptr/const_ptr.rs:804:9
note: inside `ptr::const_ptr::<impl *const ()>::sub_ptr`
  --> /checkout/library/core/src/ptr/const_ptr.rs:804:9
   |
   |
LL |         assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);
note: inside `from_ptr_range::<'_, ()>`
  --> /checkout/library/core/src/slice/raw.rs:227:42
   |
   |
LL |     unsafe { from_raw_parts(range.start, range.end.sub_ptr(range.start)) }
note: inside `R1`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:47:33
   |
   |
LL | pub static R1: &[()] = unsafe { from_ptr_range(ptr::null()..ptr::null()) };
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/const_ptr.rs:471:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:471:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: alloc107 has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
note: inside `ptr::const_ptr::<impl *const u32>::offset`
  --> /checkout/library/core/src/ptr/const_ptr.rs:471:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `ptr::const_ptr::<impl *const u32>::add`
  --> /checkout/library/core/src/ptr/const_ptr.rs:930:18
   |
LL |         unsafe { self.offset(count as isize) }
note: inside `R2`
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:50:25
   |
   |
LL |     from_ptr_range(ptr..ptr.add(2))

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:52:1
   |
   |
LL | pub static R4: &[u8] = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:57:1
  --> /checkout/src/test/ui/const-ptr/forbidden_slices.rs:57:1
   |
LL | pub static R5: &[u8] = unsafe {
---
- ...
+ note: inside `FOO`
+   --> $DIR/validate_uninhabited_zsts.rs:19:33
+    |
22 LL | const FOO: [empty::Empty; 3] = [foo(); 3];
+    |                                 ^^^^^
24 
25 error[E0080]: it is undefined behavior to use this value
26   --> $DIR/validate_uninhabited_zsts.rs:21:1
26   --> $DIR/validate_uninhabited_zsts.rs:21:1


The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/validate_uninhabited_zsts.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/validate_uninhabited_zsts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the type `!` does not permit zero-initialization
   |
LL |     unsafe { std::mem::transmute(()) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^
   |              |
   |              |
   |              this code causes undefined behavior when executed
   |              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: the `!` type has no valid value

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:4:14
   |
---
   |              ^^^^^^^^^^^^^^^^^^^^^^^
note: inside `FOO`
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:19:33
   |
LL | const FOO: [empty::Empty; 3] = [foo(); 3];

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:21:1
   |
   |
LL | const BAR: [empty::Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at [0].0: encountered a value of uninhabited type empty::Void
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 0, align: 1) {}

warning: the type `empty::Empty` does not permit zero-initialization
   |
   |
LL | const BAR: [empty::Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   |                                          |
   |                                          this code causes undefined behavior when executed
   |                                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: in this struct field
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:16:22
   |
LL |     pub struct Empty(Void);
   |                      ^^^^
note: enums with no inhabited variants have no valid value
   |
LL |     enum Void {}
   |     ^^^^^^^^^

