plain
........................................................................................  2552/14870
........................................................................................  2640/14870
........................................................................................  2728/14870
........................................................................................  2816/14870
......F...F....F........................................................................  2904/14870
......................F.................................................................  3080/14870
........................................................................................  3168/14870
.........................................................................F..............  3256/14870
........................................................................................  3344/14870
........................................................................................  3344/14870
.........F..F.........................................F.F..F.FF.........................  3432/14870
.F.................i......................................................i.............  3520/14870
........................................................................................  3696/14870
.................................................................iiiii..................  3784/14870
........................................................................................  3872/14870
........................................................................................  3960/14870
---
1 error[E0080]: evaluation of constant value failed
-   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
3    |
-    = note: dereferencing pointer failed: alloc5 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
+    = note: pointer arithmetic or comparison is not supported at compile-time
- note: inside `std::ptr::read::<u32>`
-   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- note: inside `_READ`
-   --> $DIR/out_of_bounds_read.rs:12:33
-   --> $DIR/out_of_bounds_read.rs:12:33
+ note: inside `ptr::const_ptr::<impl *const u32>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u32>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `PAST_END_PTR`
+   --> $DIR/out_of_bounds_read.rs:10:47
10    |
- LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
-    |                                 ^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     const PAST_END_PTR: *const u32 = unsafe { DATA.as_ptr().add(1) };
13 
- error[E0080]: evaluation of constant value failed
-   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+ note: erroneous constant used
+ note: erroneous constant used
+   --> $DIR/out_of_bounds_read.rs:12:43
16    |
-    = note: dereferencing pointer failed: alloc5 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
- note: inside `std::ptr::read::<u32>`
-   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- note: inside `ptr::const_ptr::<impl *const u32>::read`
-   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
- note: inside `_CONST_READ`
+ LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
+ 
+ note: erroneous constant used
24   --> $DIR/out_of_bounds_read.rs:13:39
25    |
25    |
26 LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };
-    |                                       ^^^^^^^^^^^^^^^^^^^
+    |                                       ^^^^^^^^^^^^
28 
- error[E0080]: evaluation of constant value failed
- error[E0080]: evaluation of constant value failed
-   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+ note: erroneous constant used
+   --> $DIR/out_of_bounds_read.rs:14:38
31    |
-    = note: dereferencing pointer failed: alloc5 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
- note: inside `std::ptr::read::<u32>`
-   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- note: inside `ptr::mut_ptr::<impl *mut u32>::read`
-   --> $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-   --> $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- note: inside `_MUT_READ`
-   --> $DIR/out_of_bounds_read.rs:14:37
-    |
41 LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };
+    |                                      ^^^^^^^^^^^^
43 
- error: aborting due to 3 previous errors
+ error: aborting due to previous error
---
To only update this specific test, also pass `--test-args const-ptr/out_of_bounds_read.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-ptr/out_of_bounds_read.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `PAST_END_PTR`
  --> fake-test-src-base/const-ptr/out_of_bounds_read.rs:10:47
   |
LL |     const PAST_END_PTR: *const u32 = unsafe { DATA.as_ptr().add(1) };

note: erroneous constant used
  --> fake-test-src-base/const-ptr/out_of_bounds_read.rs:12:43
   |
   |
LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };

note: erroneous constant used
  --> fake-test-src-base/const-ptr/out_of_bounds_read.rs:13:39
   |
   |
LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };

note: erroneous constant used
  --> fake-test-src-base/const-ptr/out_of_bounds_read.rs:14:38
   |
   |
LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---

71                HEX_DUMP
72            }
73 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:33:1
+ error[E0080]: could not evaluate static initializer
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
76    |
- LL | pub static S7: &[u16] = unsafe {
-    | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[1]: encountered uninitialized bytes
+    = note: pointer arithmetic or comparison is not supported at compile-time
79    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: $SIZE, align: $ALIGN) {
-                HEX_DUMP
+ note: inside `ptr::const_ptr::<impl *const u16>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u16>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `S7`
+   --> $DIR/forbidden_slices.rs:35:15
+    |
+ LL |     let ptr = (&D2 as *const Struct as *const u16).add(1);
84 
85 error[E0080]: could not evaluate static initializer
-   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
87    |
-    = note: dereferencing pointer failed: allocN has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
+    = note: pointer arithmetic or comparison is not supported at compile-time
89    |
- note: inside `std::slice::from_raw_parts::<'_, u64>`
-   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u8>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u8>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u32>::byte_add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
92 note: inside `S8`
-   --> $DIR/forbidden_slices.rs:44:5
94    |
- LL |     from_raw_parts(ptr, 1)
-    |     ^^^^^^^^^^^^^^^^^^^^^^
-    |     ^^^^^^^^^^^^^^^^^^^^^^
+ LL |     let ptr = (&D4 as *const [u32; 2] as *const u32).byte_add(1).cast::<u64>();
97 
98 error[E0080]: could not evaluate static initializer
99   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


129 error[E0080]: could not evaluate static initializer
130   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
131    |
-    = note: out-of-bounds pointer arithmetic: allocN has size 4, so pointer to 8 bytes starting at offset 0 is out-of-bounds
+    = note: pointer arithmetic or comparison is not supported at compile-time
134 note: inside `ptr::const_ptr::<impl *const u32>::offset`
135   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


141 LL |     from_ptr_range(ptr..ptr.add(2))
143 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:53:1
-   --> $DIR/forbidden_slices.rs:53:1
+ error[E0080]: could not evaluate static initializer
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
146    |
- LL | pub static R4: &[u8] = unsafe {
-    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
+    = note: pointer arithmetic or comparison is not supported at compile-time
149    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: $SIZE, align: $ALIGN) {
-                HEX_DUMP
+ note: inside `ptr::const_ptr::<impl *const u8>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u8>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `R4`
+   --> $DIR/forbidden_slices.rs:56:25
+    |
+ LL |     from_ptr_range(ptr..ptr.add(1))
154 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:58:1
-   --> $DIR/forbidden_slices.rs:58:1
+ error[E0080]: could not evaluate static initializer
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
157    |
- LL | pub static R5: &[u8] = unsafe {
-    | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    = note: pointer arithmetic or comparison is not supported at compile-time
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
-    = note: the raw bytes of the constant (size: $SIZE, align: $ALIGN) {
-                HEX_DUMP
-            }
+ note: inside `ptr::const_ptr::<impl *const &u32>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const &u32>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `R5`
+    |
+    |
+ LL |     from_ptr_range(ptr.cast()..ptr.add(1).cast())
166 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/forbidden_slices.rs:63:1
-   --> $DIR/forbidden_slices.rs:63:1
+ error[E0080]: could not evaluate static initializer
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
169    |
- LL | pub static R6: &[bool] = unsafe {
-    | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
+    = note: pointer arithmetic or comparison is not supported at compile-time
172    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: $SIZE, align: $ALIGN) {
-                HEX_DUMP
+ note: inside `ptr::const_ptr::<impl *const bool>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const bool>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `R6`
+   --> $DIR/forbidden_slices.rs:66:25
+    |
+ LL |     from_ptr_range(ptr..ptr.add(4))
177 
178 error[E0080]: could not evaluate static initializer
-   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
180    |
-    = note: accessing memory with alignment 1, but alignment 2 is required
+    = note: pointer arithmetic or comparison is not supported at compile-time
182    |
- note: inside `std::slice::from_raw_parts::<'_, u16>`
-   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
- note: inside `from_ptr_range::<'_, u16>`
-   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u8>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u8>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u16>::byte_add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
187 note: inside `R7`
-   --> $DIR/forbidden_slices.rs:70:5
189    |
189    |
- LL |     from_ptr_range(ptr..ptr.add(4))
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     let ptr = (&D2 as *const Struct as *const u16).byte_add(1);
192 
193 error[E0080]: could not evaluate static initializer
194   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


195    |
-    = note: out-of-bounds pointer arithmetic: allocN has size 8, so pointer to 8 bytes starting at offset 1 is out-of-bounds
+    = note: pointer arithmetic or comparison is not supported at compile-time
- note: inside `ptr::const_ptr::<impl *const u64>::offset`
+ note: inside `ptr::const_ptr::<impl *const u8>::offset`
199   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
- note: inside `ptr::const_ptr::<impl *const u64>::add`
---
202 note: inside `R8`
-   --> $DIR/forbidden_slices.rs:74:25
+   --> $DIR/forbidden_slices.rs:73:15
204    |
- LL |     from_ptr_range(ptr..ptr.add(1))
-    |                         ^^^^^^^^^^
+ LL |     let ptr = (&D4 as *const [u32; 2] as *const u32).byte_add(1).cast::<u64>();
207 
208 error[E0080]: could not evaluate static initializer
209   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


210    |
-    = note: `ptr_offset_from_unsigned` called on pointers into different allocations
+    = note: pointer arithmetic or comparison is not supported at compile-time
212    |
- note: inside `ptr::const_ptr::<impl *const u32>::sub_ptr`
+ note: inside `ptr::const_ptr::<impl *const u32>::offset`
214   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
- note: inside `from_ptr_range::<'_, u32>`
-   --> $SRC_DIR/core/src/slice/raw.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u32>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
217 note: inside `R9`
-   --> $DIR/forbidden_slices.rs:79:34
219    |
219    |
220 LL | pub static R9: &[u32] = unsafe { from_ptr_range(&D0..(&D0 as *const u32).add(1)) };
+    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^
222 
223 error[E0080]: could not evaluate static initializer
224   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
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

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `S7`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:35:15
   |
LL |     let ptr = (&D2 as *const Struct as *const u16).add(1);

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:940:18
note: inside `S8`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:42:15
   |
LL |     let ptr = (&D4 as *const [u32; 2] as *const u32).byte_add(1).cast::<u64>();

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:798:18
   |
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
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R2`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:51:25
   |
LL |     from_ptr_range(ptr..ptr.add(2))

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R4`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:56:25
   |
LL |     from_ptr_range(ptr..ptr.add(1))

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const &u32>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `ptr::const_ptr::<impl *const &u32>::add`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R5`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:61:32
   |
LL |     from_ptr_range(ptr.cast()..ptr.add(1).cast())

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R6`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:66:25
   |
LL |     from_ptr_range(ptr..ptr.add(4))

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:940:18
note: inside `R7`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:69:15
   |
LL |     let ptr = (&D2 as *const Struct as *const u16).byte_add(1);

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:940:18
note: inside `R8`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:73:15
   |
LL |     let ptr = (&D4 as *const [u32; 2] as *const u32).byte_add(1).cast::<u64>();

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R9`
  --> fake-test-src-base/const-ptr/forbidden_slices.rs:79:54
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
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] tests/ui/const-ptr/allowed_slices.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-ptr/allowed_slices.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/allowed_slices/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/allowed_slices/auxiliary"
stdout: none
error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:940:18
note: inside `S7`
  --> fake-test-src-base/const-ptr/allowed_slices.rs:35:15
   |
LL |     let ptr = (&D2 as *const Struct as *const u16).byte_add(4);

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R2`
  --> fake-test-src-base/const-ptr/allowed_slices.rs:50:25
   |
LL |     from_ptr_range(ptr..ptr.add(1))

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const MaybeUninit<&u32>>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `ptr::const_ptr::<impl *const MaybeUninit<&u32>>::add`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R3`
  --> fake-test-src-base/const-ptr/allowed_slices.rs:54:25
   |
LL |     from_ptr_range(ptr..ptr.add(1))

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R4`
  --> fake-test-src-base/const-ptr/allowed_slices.rs:58:25
   |
LL |     from_ptr_range(ptr..ptr.add(3))

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const MaybeUninit<u8>>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `ptr::const_ptr::<impl *const MaybeUninit<u8>>::add`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R5`
  --> fake-test-src-base/const-ptr/allowed_slices.rs:62:25
   |
LL |     from_ptr_range(ptr..ptr.add(2))

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R6`
  --> fake-test-src-base/const-ptr/allowed_slices.rs:66:25
   |
LL |     from_ptr_range(ptr..ptr.add(4))

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/slice/mod.rs:557:28
note: inside `R9`
  --> fake-test-src-base/const-ptr/allowed_slices.rs:84:49
   |
LL | pub static R9: &[u32] = unsafe { from_ptr_range(R0.as_ptr_range()) };

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0080`.
---

546                ╾ALLOC_ID╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
547            }
548 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/raw-bytes.rs:225:1
+ error[E0080]: could not evaluate static initializer
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
551    |
- LL | pub static S7: &[u16] = unsafe {
-    | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[1]: encountered uninitialized bytes
+    = note: pointer arithmetic or comparison is not supported at compile-time
554    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾ALLOC_ID+0x2╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
+ note: inside `ptr::const_ptr::<impl *const u16>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u16>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `S7`
+   --> $DIR/raw-bytes.rs:227:15
+    |
+ LL |     let ptr = (&D2 as *const Struct as *const u16).add(1);
559 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/raw-bytes.rs:232:1
-   --> $DIR/raw-bytes.rs:232:1
+ error[E0080]: could not evaluate static initializer
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
562    |
- LL | pub static R4: &[u8] = unsafe {
-    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered uninitialized bytes
+    = note: pointer arithmetic or comparison is not supported at compile-time
565    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾ALLOC_ID╼ 01 00 00 00 00 00 00 00 │ ╾──────╼........
+ note: inside `ptr::const_ptr::<impl *const u8>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u8>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `R4`
+   --> $DIR/raw-bytes.rs:235:25
+    |
+ LL |     from_ptr_range(ptr..ptr.add(1))
570 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/raw-bytes.rs:237:1
-   --> $DIR/raw-bytes.rs:237:1
+ error[E0080]: could not evaluate static initializer
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
573    |
- LL | pub static R5: &[u8] = unsafe {
-    | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    = note: pointer arithmetic or comparison is not supported at compile-time
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
-    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾ALLOC_ID╼ 08 00 00 00 00 00 00 00 │ ╾──────╼........
-            }
+ note: inside `ptr::const_ptr::<impl *const &u32>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const &u32>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `R5`
+    |
+    |
+ LL |     from_ptr_range(ptr.cast()..ptr.add(1).cast())
582 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/raw-bytes.rs:242:1
-   --> $DIR/raw-bytes.rs:242:1
+ error[E0080]: could not evaluate static initializer
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
585    |
- LL | pub static R6: &[bool] = unsafe {
-    | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x11, but expected a boolean
+    = note: pointer arithmetic or comparison is not supported at compile-time
588    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾ALLOC_ID╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
+ note: inside `ptr::const_ptr::<impl *const bool>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const bool>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `R6`
+   --> $DIR/raw-bytes.rs:245:25
+    |
+ LL |     from_ptr_range(ptr..ptr.add(4))
593 
594 error: aborting due to 52 previous errors
595 



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/raw-bytes/raw-bytes.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/raw-bytes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/raw-bytes.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/raw-bytes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/raw-bytes/auxiliary"
stdout: none
error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:20:1
   |
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000001, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:28:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:28:1
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000000, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:42:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:42:1
   |
LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(B)>.0: encountered a value of the never type `!`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:44:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:44:1
   |
LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:50:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:50:1
   |
LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Some)>.0.1: encountered 0xffffffff, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               78 00 00 00 ff ff ff ff                         │ x.......

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:54:1
   |
   |
LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:57:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:57:1
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:59:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:59:1
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:65:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:65:1
   |
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 42, but expected something in the range 10..=30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               2a 00 00 00                                     │ *...

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:71:1
   |
   |
LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 20, but expected something less or equal to 10, or greater or equal to 30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:74:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:74:1
   |
LL | const NULL_FAT_PTR: NonNull<dyn Send> = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               00 00 00 00 00 00 00 00 ╾───────alloc28───────╼ │ ........╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:82:1
   |
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:86:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:86:1
   |
LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned box (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:90:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:90:1
   |
LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^ constructing invalid value: encountered a null reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:93:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:93:1
   |
LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a null box
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:96:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:96:1
   |
LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (0x539[noalloc] has no provenance)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:99:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:99:1
   |
LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling box (0x539[noalloc] has no provenance)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:102:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:102:1
   |
LL | const NULL_FN_PTR: fn() = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered null pointer, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:104:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:104:1
   |
LL | const DANGLING_FN_PTR: fn() = unsafe { mem::transmute(13usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0xd[noalloc], but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               0d 00 00 00 00 00 00 00                         │ ........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:106:1
   |
   |
LL | const DATA_FN_PTR: fn() = unsafe { mem::transmute(&13) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered alloc55, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:112:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:112:1
   |
LL | const BAD_BAD_REF: &Bar = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to uninhabited type Bar
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:137:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:137:1
   |
LL | const STR_TOO_LONG: &str = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc61───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:139:1
   |
   |
LL | const NESTED_STR_MUCH_TOO_LONG: (&str,) = (unsafe { mem::transmute((&42, usize::MAX)) },);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc67───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:141:1
   |
   |
LL | const MY_STR_MUCH_TOO_LONG: &MyStr = unsafe { mem::transmute((&42u8, usize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc73───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:144:1
   |
   |
LL | const STR_NO_INIT: &str = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:146:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:146:1
   |
LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.0: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:148:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:148:1
   |
LL | const MYSTR_NO_INIT_ISSUE83182: &MyStr = unsafe { mem::transmute::<&[_], _>(&[&()]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:152:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:152:1
   |
LL | const SLICE_TOO_LONG: &[u8] = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc93───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:154:1
   |
   |
LL | const SLICE_TOO_LONG_OVERFLOW: &[u32] = unsafe { mem::transmute((&42u32, isize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc99───────╼ ff ff ff ff ff ff ff 7f │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:157:1
   |
   |
LL | const SLICE_TOO_LONG_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling box (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────alloc104───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:160:1
   |
   |
LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

note: erroneous constant used
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:160:40
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:160:40
   |
LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:166:1
   |
   |
LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.0: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

note: erroneous constant used
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:166:42
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:166:42
   |
LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:170:1
   |
   |
LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.1[0]: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

note: erroneous constant used
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:170:42
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:170:42
   |
LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:175:1
   |
   |
LL | const TRAIT_OBJ_SHORT_VTABLE_1: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u8))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered alloc125, but expected a vtable pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `S7`
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:227:15
   |
LL |     let ptr = (&D2 as *const Struct as *const u16).add(1);

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R4`
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:235:25
   |
LL |     from_ptr_range(ptr..ptr.add(1))

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const &u32>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `ptr::const_ptr::<impl *const &u32>::add`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R5`
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:240:32
   |
LL |     from_ptr_range(ptr.cast()..ptr.add(1).cast())

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `R6`
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:245:25
   |
LL |     from_ptr_range(ptr..ptr.add(4))

error: aborting due to 52 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] tests/ui/consts/invalid-union.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/invalid-union.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:479:18
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:1020:18
note: inside `C`
  --> fake-test-src-base/consts/invalid-union.rs:37:15
   |
LL |     unsafe { *p.add(1) = 1 };

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] tests/ui/consts/issue-94371.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/issue-94371.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-94371" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-94371/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:479:18
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::mut_ptr::<impl *mut MaybeUninit<usize>>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:479:18
note: inside `ptr::mut_ptr::<impl *mut MaybeUninit<usize>>::add`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:1020:18
note: inside `ptr::swap_nonoverlapping_simple_untyped::<usize>`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:962:32
note: inside `swap_nonoverlapping::<Demo>`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:942:9
note: inside `std::mem::swap::<Demo>`
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:744:29
  --> fake-test-src-base/consts/issue-94371.rs:12:5
   |
   |
LL |     std::mem::swap(&mut x, &mut y);
   = note: this error originates in the macro `attempt_swap_as_chunks` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
1 error[E0080]: evaluation of constant value failed
-   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
3    |
-    = note: unable to turn pointer into raw bytes
+    = note: pointer arithmetic or comparison is not supported at compile-time
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
- note: inside `std::ptr::read::<u8>`
-   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- note: inside `ptr::const_ptr::<impl *const u8>::read`
+ note: inside `ptr::const_ptr::<impl *const u8>::offset`
11   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const u8>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
13   --> $DIR/issue-miri-1910.rs:8:5
14    |


15 LL |     (&foo as *const _ as *const u8).add(one_and_a_half_pointers).read();
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
17 
18 error: aborting due to previous error
19 
---
To only update this specific test, also pass `--test-args consts/issue-miri-1910.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/issue-miri-1910.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-miri-1910" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-miri-1910/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `C`
  --> fake-test-src-base/consts/issue-miri-1910.rs:8:5
   |
LL |     (&foo as *const _ as *const u8).add(one_and_a_half_pointers).read();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
3    |
-    = note: unable to copy parts of a pointer from memory at ALLOC_ID
+    = note: pointer arithmetic or comparison is not supported at compile-time
5    |
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
- note: inside `std::ptr::read::<MaybeUninit<MaybeUninit<u8>>>`
-   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- note: inside `mem::swap_simple::<MaybeUninit<MaybeUninit<u8>>>`
-   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
+ note: inside `ptr::mut_ptr::<impl *mut MaybeUninit<MaybeUninit<u8>>>::offset`
+   --> $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+ note: inside `ptr::mut_ptr::<impl *mut MaybeUninit<MaybeUninit<u8>>>::add`
+   --> $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
12 note: inside `ptr::swap_nonoverlapping_simple_untyped::<MaybeUninit<u8>>`
13   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
14 note: inside `swap_nonoverlapping::<MaybeUninit<u8>>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/missing_span_in_backtrace/missing_span_in_backtrace.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/missing_span_in_backtrace.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/missing_span_in_backtrace.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/missing_span_in_backtrace" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/missing_span_in_backtrace/auxiliary" "-Z" "ui-testing=no"
stdout: none
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:479:18
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::mut_ptr::<impl *mut MaybeUninit<MaybeUninit<u8>>>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:479:18
note: inside `ptr::mut_ptr::<impl *mut MaybeUninit<MaybeUninit<u8>>>::add`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:1020:18
note: inside `ptr::swap_nonoverlapping_simple_untyped::<MaybeUninit<u8>>`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:962:32
note: inside `swap_nonoverlapping::<MaybeUninit<u8>>`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:947:14
note: inside `X`
  --> fake-test-src-base/consts/missing_span_in_backtrace.rs:17:9
17 | /         ptr::swap_nonoverlapping(
17 | /         ptr::swap_nonoverlapping(
18 | |             &mut ptr1 as *mut _ as *mut MaybeUninit<u8>,
19 | |             &mut ptr2 as *mut _ as *mut MaybeUninit<u8>,
20 | |             mem::size_of::<&i32>(),
   | |_________^

error: aborting due to previous error

---
---- [ui] tests/ui/consts/offset_from.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/offset_from.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `OFFSET_UNSIGNED`
  --> fake-test-src-base/consts/offset_from.rs:50:14
   |
LL |     unsafe { ptr.add(2).sub_ptr(ptr) }

note: erroneous constant used
  --> fake-test-src-base/consts/offset_from.rs:58:16
   |
---
---- [ui] tests/ui/consts/offset.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/offset.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset/auxiliary"
stdout: none
error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u32>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `OFFSET_NO_CHANGE`
  --> fake-test-src-base/consts/offset.rs:17:14
   |
LL |     let p2 = p1.offset(2).offset(-2);

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u32>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `OFFSET_MIDDLE`
  --> fake-test-src-base/consts/offset.rs:21:14
   |
LL |     let p1 = (&S.a as *const u32).offset(1);

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u32>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `OFFSET_END`
  --> fake-test-src-base/consts/offset.rs:27:14
   |
LL |     let p1 = (&S.a as *const u32).offset(3);

error[E0080]: could not evaluate static initializer
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `OFFSET_U8_PTR`
  --> fake-test-src-base/consts/offset.rs:33:14
   |
LL |     let p1 = (&S.a as *const u32 as *const u8).offset(5);

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const ()>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `OFFSET_ZST`
  --> fake-test-src-base/consts/offset.rs:41:14
   |
LL |     let p1 = pz.offset(5) as *const u8;

note: erroneous constant used
  --> fake-test-src-base/consts/offset.rs:101:13
   |
   |
LL |     assert!(OFFSET_ZST);

note: erroneous constant used
  --> fake-test-src-base/consts/offset.rs:101:13
   |
   |
LL |     assert!(OFFSET_ZST);

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `OFFSET_ZERO`
  --> fake-test-src-base/consts/offset.rs:47:5
   |
LL |     p.offset(0).offset_from(p) == 0

note: erroneous constant used
  --> fake-test-src-base/consts/offset.rs:102:13
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `OFFSET_ONE`
  --> fake-test-src-base/consts/offset.rs:51:5
   |
LL |     p.offset(1).offset_from(p) == 1

note: erroneous constant used
  --> fake-test-src-base/consts/offset.rs:103:13
   |
---
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:479:18
note: inside `OFFSET_DANGLING`
  --> fake-test-src-base/consts/offset.rs:55:5
   |
LL |     p.offset(0).offset_from(p) == 0

note: erroneous constant used
  --> fake-test-src-base/consts/offset.rs:104:13
   |
---

note: erroneous constant used
  --> fake-test-src-base/consts/offset.rs:105:13
   |
LL |     assert!(OFFSET_UNALIGNED);

note: erroneous constant used
  --> fake-test-src-base/consts/offset.rs:105:13
   |
   |
LL |     assert!(OFFSET_UNALIGNED);

error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0080`.
---

14 error[E0080]: evaluation of constant value failed
15   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
16    |
-    = note: out-of-bounds pointer arithmetic: allocN has size 1, so pointer to 2 bytes starting at offset 0 is out-of-bounds
+    = note: pointer arithmetic or comparison is not supported at compile-time
19 note: inside `ptr::const_ptr::<impl *const u8>::offset`
20   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

27 error[E0080]: evaluation of constant value failed
27 error[E0080]: evaluation of constant value failed
28   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
29    |
-    = note: out-of-bounds pointer arithmetic: allocN has size 100, so pointer to 101 bytes starting at offset 0 is out-of-bounds
+    = note: pointer arithmetic or comparison is not supported at compile-time
32 note: inside `ptr::const_ptr::<impl *const u8>::offset`
33   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

40 error[E0080]: evaluation of constant value failed
---

92 error[E0080]: evaluation of constant value failed
93   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
94    |
-    = note: out-of-bounds pointer arithmetic: allocN has size 1, so pointer to 2 bytes starting at offset -4 is out-of-bounds
+    = note: pointer arithmetic or comparison is not supported at compile-time
97 note: inside `ptr::const_ptr::<impl *const u8>::offset`
98   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

105 error[E0080]: evaluation of constant value failed
105 error[E0080]: evaluation of constant value failed
106   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
107    |
-    = note: out-of-bounds pointer arithmetic: allocN has size 0, so pointer to 1 byte starting at offset 0 is out-of-bounds
+    = note: pointer arithmetic or comparison is not supported at compile-time
110 note: inside `ptr::const_ptr::<impl *const u8>::offset`
111   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

118 error[E0080]: evaluation of constant value failed
118 error[E0080]: evaluation of constant value failed
119   --> $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
120    |
-    = note: out-of-bounds pointer arithmetic: 0x1[noalloc] is a dangling pointer (it has no provenance)
+    = note: pointer arithmetic or comparison is not supported at compile-time
123 note: inside `ptr::mut_ptr::<impl *mut u8>::offset`
124   --> $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL

131 error[E0080]: evaluation of constant value failed
131 error[E0080]: evaluation of constant value failed
132   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
133    |
-    = note: out-of-bounds pointer arithmetic: null pointer is a dangling pointer (it has no provenance)
+    = note: pointer arithmetic or comparison is not supported at compile-time
136 note: inside `ptr::const_ptr::<impl *const u8>::offset`
137   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

144 error[E0080]: evaluation of constant value failed
144 error[E0080]: evaluation of constant value failed
145   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
146    |
-    = note: out-of-bounds pointer arithmetic: 0x7f..f[noalloc] is a dangling pointer (it has no provenance)
+    = note: pointer arithmetic or comparison is not supported at compile-time
149 note: inside `ptr::const_ptr::<impl *const u8>::offset`
150   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/offset_ub.stderr
To only update this specific test, also pass `--test-args consts/offset_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/offset_ub.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `BEFORE_START`
  --> fake-test-src-base/consts/offset_ub.rs:7:46
   |
LL | pub const BEFORE_START: *const u8 = unsafe { (&0u8 as *const u8).offset(-1) }; //~NOTE

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `AFTER_END`
  --> fake-test-src-base/consts/offset_ub.rs:8:43
   |
LL | pub const AFTER_END: *const u8 = unsafe { (&0u8 as *const u8).offset(2) }; //~NOTE

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `AFTER_ARRAY`
  --> fake-test-src-base/consts/offset_ub.rs:9:45
   |
LL | pub const AFTER_ARRAY: *const u8 = unsafe { [0u8; 100].as_ptr().offset(101) }; //~NOTE

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u16>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `OVERFLOW`
  --> fake-test-src-base/consts/offset_ub.rs:11:43
   |
LL | pub const OVERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MAX) }; //~NOTE

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u16>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `UNDERFLOW`
  --> fake-test-src-base/consts/offset_ub.rs:12:44
   |
LL | pub const UNDERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MIN) }; //~NOTE

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `OVERFLOW_ADDRESS_SPACE`
  --> fake-test-src-base/consts/offset_ub.rs:13:56
   |
LL | pub const OVERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (usize::MAX as *const u8).offset(2) }; //~NOTE

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `UNDERFLOW_ADDRESS_SPACE`
  --> fake-test-src-base/consts/offset_ub.rs:14:57
   |
LL | pub const UNDERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (1 as *const u8).offset(-2) }; //~NOTE

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `NEGATIVE_OFFSET`
  --> fake-test-src-base/consts/offset_ub.rs:15:49
   |
LL | pub const NEGATIVE_OFFSET: *const u8 = unsafe { [0u8; 1].as_ptr().wrapping_offset(-2).offset(-2) }; //~NOTE

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `ZERO_SIZED_ALLOC`
  --> fake-test-src-base/consts/offset_ub.rs:17:50
   |
LL | pub const ZERO_SIZED_ALLOC: *const u8 = unsafe { [0u8; 0].as_ptr().offset(1) }; //~NOTE

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:479:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::mut_ptr::<impl *mut u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mut_ptr.rs:479:18
note: inside `DANGLING`
  --> fake-test-src-base/consts/offset_ub.rs:18:42
   |
LL | pub const DANGLING: *const u8 = unsafe { ptr::NonNull::<u8>::dangling().as_ptr().offset(4) }; //~NOTE

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `NULL_OFFSET_ZERO`
  --> fake-test-src-base/consts/offset_ub.rs:21:50
   |
LL | pub const NULL_OFFSET_ZERO: *const u8 = unsafe { ptr::null::<u8>().offset(0) }; //~NOTE

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `UNDERFLOW_ABS`
  --> fake-test-src-base/consts/offset_ub.rs:24:47
   |
LL | pub const UNDERFLOW_ABS: *const u8 = unsafe { (usize::MAX as *const u8).offset(isize::MIN) }; //~NOTE

error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] tests/ui/consts/offset_from_ub.rs stdout ----
diff of stderr:

72    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from` called when first pointer is too far before second
74 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:99:14
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
76    |
76    |
+    = note: pointer arithmetic or comparison is not supported at compile-time
+    |
+ note: inside `ptr::const_ptr::<impl *const char>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `ptr::const_ptr::<impl *const char>::add`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `WRONG_ORDER_UNSIGNED`
+    |
+    |
77 LL |     unsafe { ptr_offset_from_unsigned(p, p.add(2) ) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called when first pointer has smaller offset than second: 0 < 8
79 
80 error[E0080]: evaluation of constant value failed
81   --> $DIR/offset_from_ub.rs:106:14



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/offset_from_ub.rs:18:27
   |
   |
LL |     let offset = unsafe { ptr_offset_from(field_ptr, base_ptr) }; //~ERROR evaluation of constant value failed
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from` called on pointers into different allocations
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:696:18
   |
   = note: `ptr_offset_from` called on pointers into different allocations
   = note: `ptr_offset_from` called on pointers into different allocations
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset_from`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:696:18
note: inside `NOT_PTR`
  --> fake-test-src-base/consts/offset_from_ub.rs:24:14
   |
LL |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/offset_from_ub.rs:31:14
   |
   |
LL |     unsafe { ptr_offset_from(field_ptr, base_ptr as *const u16) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exact_div: 1_isize cannot be divided by 2_isize without remainder
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/offset_from_ub.rs:37:14
   |
   |
LL |     unsafe { ptr_offset_from(ptr, ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/offset_from_ub.rs:44:14
   |
   |
LL |     unsafe { ptr_offset_from(ptr2, ptr1) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: 0x8[noalloc] is a dangling pointer (it has no provenance)
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/offset_from_ub.rs:53:14
   |
   |
LL |     unsafe { ptr_offset_from(end_ptr, start_ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc17 has size 4, so pointer to 10 bytes starting at offset 0 is out-of-bounds
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/offset_from_ub.rs:62:14
   |
   |
LL |     unsafe { ptr_offset_from(start_ptr, end_ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc20 has size 4, so pointer to 10 bytes starting at offset 0 is out-of-bounds
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/offset_from_ub.rs:70:14
   |
   |
LL |     unsafe { ptr_offset_from(end_ptr, end_ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc23 has size 4, so pointer at offset 10 is out-of-bounds
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/offset_from_ub.rs:79:14
   |
   |
LL |     unsafe { ptr_offset_from_unsigned(field_ptr, base_ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called on pointers into different allocations
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/offset_from_ub.rs:86:14
   |
   |
LL |     unsafe { ptr_offset_from(ptr2, ptr1) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from` called when first pointer is too far ahead of second
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/offset_from_ub.rs:92:14
   |
   |
LL |     unsafe { ptr_offset_from(ptr1, ptr2) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from` called when first pointer is too far before second
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const char>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `ptr::const_ptr::<impl *const char>::add`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:920:18
note: inside `WRONG_ORDER_UNSIGNED`
  --> fake-test-src-base/consts/offset_from_ub.rs:99:42
   |
LL |     unsafe { ptr_offset_from_unsigned(p, p.add(2) ) } //~ERROR evaluation of constant value failed

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/offset_from_ub.rs:106:14
   |
   |
LL |     unsafe { ptr_offset_from_unsigned(ptr2, ptr1) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called when first pointer is too far ahead of second
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:696:18
   |
   = note: out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   = note: out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset_from`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:696:18
note: inside `OFFSET_VERY_FAR1`
  --> fake-test-src-base/consts/offset_from_ub.rs:115:14
LL |     unsafe { ptr2.offset_from(ptr1) }
   |              ^^^^^^^^^^^^^^^^^^^^^^

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:696:18
   |
   = note: out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset_from`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:696:18
note: inside `OFFSET_VERY_FAR2`
  --> fake-test-src-base/consts/offset_from_ub.rs:121:14
   |
LL |     unsafe { ptr1.offset_from(ptr2.wrapping_offset(1)) }

error: aborting due to 15 previous errors

For more information about this error, try `rustc --explain E0080`.
---

1 error[E0080]: evaluation of constant value failed
2   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
3    |
-    = note: out-of-bounds pointer arithmetic: alloc3 has size $WORD, so pointer to $TWO_WORDS bytes starting at offset 0 is out-of-bounds
+    = note: pointer arithmetic or comparison is not supported at compile-time
6 note: inside `ptr::const_ptr::<impl *const usize>::offset`
7   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

8 note: inside `_`
8 note: inside `_`
+   --> $DIR/ptr_comparisons.rs:32:21
+    |
+ LL | check!(ne, unsafe { (FOO as *const usize).offset(1) }, 0);
+ 
+ error[E0080]: evaluation of constant value failed
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+    |
+    = note: pointer arithmetic or comparison is not supported at compile-time
+    |
+ note: inside `ptr::const_ptr::<impl *const u8>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `_`
+   --> $DIR/ptr_comparisons.rs:33:21
+    |
+ LL | check!(ne, unsafe { (FOO as *const usize as *const u8).offset(3) }, 0);
+ 
+ error[E0080]: evaluation of constant value failed
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+    |
+    = note: pointer arithmetic or comparison is not supported at compile-time
+    |
+ note: inside `ptr::const_ptr::<impl *const usize>::offset`
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ note: inside `_`
9   --> $DIR/ptr_comparisons.rs:50:34
10    |
11 LL | const _: *const usize = unsafe { (FOO as *const usize).offset(2) };
35    = help: this code performed an operation that depends on the underlying bytes representing a pointer
35    = help: this code performed an operation that depends on the underlying bytes representing a pointer
36    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
- error: aborting due to 4 previous errors
+ error: aborting due to 6 previous errors
39 
40 For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/ptr_comparisons.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/ptr_comparisons.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/auxiliary" "--crate-type=lib"
stdout: none
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const usize>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `_`
  --> fake-test-src-base/consts/ptr_comparisons.rs:32:21
   |
LL | check!(ne, unsafe { (FOO as *const usize).offset(1) }, 0);

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const u8>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `_`
  --> fake-test-src-base/consts/ptr_comparisons.rs:33:21
   |
LL | check!(ne, unsafe { (FOO as *const usize as *const u8).offset(3) }, 0);

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
   |
   |
   = note: pointer arithmetic or comparison is not supported at compile-time
   |
note: inside `ptr::const_ptr::<impl *const usize>::offset`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/const_ptr.rs:465:18
note: inside `_`
  --> fake-test-src-base/consts/ptr_comparisons.rs:50:34
   |
LL | const _: *const usize = unsafe { (FOO as *const usize).offset(2) };

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/ptr_comparisons.rs:53:33
   |
   |
LL |     unsafe { std::ptr::addr_of!((*(FOO as *const usize as *const [u8; 1000]))[999]) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: alloc3 has size 8, so pointer to 1000 bytes starting at offset 0 is out-of-bounds
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/ptr_comparisons.rs:57:27
   |
   |
LL | const _: usize = unsafe { std::mem::transmute::<*const usize, usize>(FOO) + 4 };
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/ptr_comparisons.rs:61:27
   |
   |
LL | const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
