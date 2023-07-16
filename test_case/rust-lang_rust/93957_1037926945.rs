plain
..........................................F......................................................... 2300/12631
.................................................................................................... 2400/12631
.................................................................................................... 2500/12631
.................................................................................................... 2600/12631
............................................F....................................................... 2700/12631
.................................F...................................................F.............. 2800/12631
...F..........................................................................i..................... 2900/12631
..............................................................................................iiiii. 3100/12631
.................................................................................................... 3200/12631
.................................................................................................... 3300/12631
.................................................................................................... 3400/12631
---

---- [ui] ui/const-ptr/out_of_bounds_read.rs stdout ----
diff of stderr:

7    |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
8    |         inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-   ::: $DIR/out_of_bounds_read.rs:13:33
+   ::: $DIR/out_of_bounds_read.rs:12:33
11    |
11    |
12 LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
-    |                                 ----------------------- inside `_READ` at $DIR/out_of_bounds_read.rs:13:33
+    |                                 ----------------------- inside `_READ` at $DIR/out_of_bounds_read.rs:12:33
15 error[E0080]: evaluation of constant value failed
16   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


26 LL |         unsafe { read(self) }
27    |                  ---------- inside `ptr::const_ptr::<impl *const u32>::read` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/out_of_bounds_read.rs:14:39
+   ::: $DIR/out_of_bounds_read.rs:13:39
30    |
30    |
31 LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };
-    |                                       ------------------- inside `_CONST_READ` at $DIR/out_of_bounds_read.rs:14:39
+    |                                       ------------------- inside `_CONST_READ` at $DIR/out_of_bounds_read.rs:13:39
34 error[E0080]: evaluation of constant value failed
35   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL


45 LL |         unsafe { read(self) }
46    |                  ---------- inside `ptr::mut_ptr::<impl *mut u32>::read` at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-   ::: $DIR/out_of_bounds_read.rs:15:37
+   ::: $DIR/out_of_bounds_read.rs:14:37
49    |
49    |
50 LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };
-    |                                     --------------------------------- inside `_MUT_READ` at $DIR/out_of_bounds_read.rs:15:37
+    |                                     --------------------------------- inside `_MUT_READ` at $DIR/out_of_bounds_read.rs:14:37
53 error: aborting due to 3 previous errors
54 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/out_of_bounds_read.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-ptr/out_of_bounds_read.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-ptr/out_of_bounds_read.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mod.rs:701:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:701:9
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:12:33
   |
   |
LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
   |                                 ----------------------- inside `_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:12:33
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mod.rs:701:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:701:9
  ::: /checkout/library/core/src/ptr/const_ptr.rs:822:18
   |
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::const_ptr::<impl *const u32>::read` at /checkout/library/core/src/ptr/const_ptr.rs:822:18
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:13:39
   |
   |
LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };
   |                                       ------------------- inside `_CONST_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:13:39
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mod.rs:701:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:701:9
  ::: /checkout/library/core/src/ptr/mut_ptr.rs:936:18
   |
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::mut_ptr::<impl *mut u32>::read` at /checkout/library/core/src/ptr/mut_ptr.rs:936:18
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:37
   |
   |
LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };
   |                                     --------------------------------- inside `_MUT_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:37
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.

---
1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/invalid-union.rs:41:1
+   --> $DIR/invalid-union.rs:40:1
3    |
4 LL | fn main() {
5    | ^^^^^^^^^ type validation failed at .<deref>.y.<enum-variant(B)>.0: encountered `UnsafeCell` in a `const`
10            }
11 
12 error: erroneous constant used
-   --> $DIR/invalid-union.rs:42:25
-   --> $DIR/invalid-union.rs:42:25
+   --> $DIR/invalid-union.rs:41:25
14    |
15 LL |     let _: &'static _ = &C;
16    |                         ^^ referenced constant has errors

The actual 64bit.stderr differed from the expected 64bit.stderr.
The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/invalid-union.64bit.stderr
To only update this specific test, also pass `--test-args consts/invalid-union.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/invalid-union.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/invalid-union.rs:40:1
   |
LL | fn main() { //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^ type validation failed at .<deref>.y.<enum-variant(B)>.0: encountered `UnsafeCell` in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: erroneous constant used
  --> /checkout/src/test/ui/consts/invalid-union.rs:41:25
  --> /checkout/src/test/ui/consts/invalid-union.rs:41:25
   |
LL |     let _: &'static _ = &C; //~ ERROR erroneous constant used
   |                         ^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 2 previous errors

---

---- [ui] ui/consts/issue-miri-1910.rs stdout ----
diff of stderr:

7    |           unable to turn pointer into raw bytes
8    |           inside `std::ptr::read::<u8>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
9    |           inside `ptr::const_ptr::<impl *const u8>::read` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |           inside `C` at $DIR/issue-miri-1910.rs:8:5
+    |           inside `C` at $DIR/issue-miri-1910.rs:7:5
-   ::: $DIR/issue-miri-1910.rs:5:1
+   ::: $DIR/issue-miri-1910.rs:4:1
13    |
13    |
14 LL | / const C: () = unsafe {
15 LL | |     let foo = Some(&42 as *const i32);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-miri-1910/issue-miri-1910.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-miri-1910.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-miri-1910.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-miri-1910" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-miri-1910/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/mod.rs:701:9
   |
LL |           copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |           |
   |           unable to turn pointer into raw bytes
   |           inside `std::ptr::read::<u8>` at /checkout/library/core/src/ptr/mod.rs:701:9
   |           inside `ptr::const_ptr::<impl *const u8>::read` at /checkout/library/core/src/ptr/const_ptr.rs:822:18
   |           inside `C` at /checkout/src/test/ui/consts/issue-miri-1910.rs:7:5
  ::: /checkout/src/test/ui/consts/issue-miri-1910.rs:4:1
   |
   |
LL | / const C: () = unsafe {
LL | |     let foo = Some(&42 as *const i32);
LL | |     let one_and_a_half_pointers = std::mem::size_of::<*const i32>()/2*3;
LL | |     (&foo as *const _ as *const u8).add(one_and_a_half_pointers).read();
LL | | };
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/offset_ub.rs stdout ----
diff of stderr:

7    |                  overflowing in-bounds pointer arithmetic
8    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:8:46
+   ::: $DIR/offset_ub.rs:7:46
11    |
11    |
12 LL | pub const BEFORE_START: *const u8 = unsafe { (&0u8 as *const u8).offset(-1) };
-    |                                              ------------------------------ inside `BEFORE_START` at $DIR/offset_ub.rs:8:46
+    |                                              ------------------------------ inside `BEFORE_START` at $DIR/offset_ub.rs:7:46
15 error[E0080]: evaluation of constant value failed
16   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


21    |                  pointer arithmetic failed: allocN has size 1, so pointer to 2 bytes starting at offset 0 is out-of-bounds
22    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:9:43
+   ::: $DIR/offset_ub.rs:8:43
25    |
25    |
26 LL | pub const AFTER_END: *const u8 = unsafe { (&0u8 as *const u8).offset(2) };
-    |                                           ----------------------------- inside `AFTER_END` at $DIR/offset_ub.rs:9:43
+    |                                           ----------------------------- inside `AFTER_END` at $DIR/offset_ub.rs:8:43
29 error[E0080]: evaluation of constant value failed
30   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


35    |                  pointer arithmetic failed: allocN has size 100, so pointer to 101 bytes starting at offset 0 is out-of-bounds
36    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:10:45
+   ::: $DIR/offset_ub.rs:9:45
39    |
39    |
40 LL | pub const AFTER_ARRAY: *const u8 = unsafe { [0u8; 100].as_ptr().offset(101) };
-    |                                             ------------------------------- inside `AFTER_ARRAY` at $DIR/offset_ub.rs:10:45
+    |                                             ------------------------------- inside `AFTER_ARRAY` at $DIR/offset_ub.rs:9:45
43 error[E0080]: evaluation of constant value failed
44   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

49    |                  overflowing in-bounds pointer arithmetic
49    |                  overflowing in-bounds pointer arithmetic
50    |                  inside `ptr::const_ptr::<impl *const u16>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:12:43
+   ::: $DIR/offset_ub.rs:11:43
53    |
53    |
54 LL | pub const OVERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MAX) };
-    |                                           ------------------------------------- inside `OVERFLOW` at $DIR/offset_ub.rs:12:43
+    |                                           ------------------------------------- inside `OVERFLOW` at $DIR/offset_ub.rs:11:43
57 error[E0080]: evaluation of constant value failed
58   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

63    |                  overflowing in-bounds pointer arithmetic
63    |                  overflowing in-bounds pointer arithmetic
64    |                  inside `ptr::const_ptr::<impl *const u16>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:13:44
+   ::: $DIR/offset_ub.rs:12:44
67    |
67    |
68 LL | pub const UNDERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MIN) };
-    |                                            ------------------------------------- inside `UNDERFLOW` at $DIR/offset_ub.rs:13:44
+    |                                            ------------------------------------- inside `UNDERFLOW` at $DIR/offset_ub.rs:12:44
71 error[E0080]: evaluation of constant value failed
72   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

77    |                  overflowing in-bounds pointer arithmetic
77    |                  overflowing in-bounds pointer arithmetic
78    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:14:56
+   ::: $DIR/offset_ub.rs:13:56
81    |
81    |
82 LL | pub const OVERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (usize::MAX as *const u8).offset(2) };
-    |                                                        ----------------------------------- inside `OVERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:14:56
+    |                                                        ----------------------------------- inside `OVERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:13:56
85 error[E0080]: evaluation of constant value failed
86   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

91    |                  overflowing in-bounds pointer arithmetic
91    |                  overflowing in-bounds pointer arithmetic
92    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:15:57
+   ::: $DIR/offset_ub.rs:14:57
95    |
95    |
96 LL | pub const UNDERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (1 as *const u8).offset(-2) };
-    |                                                         --------------------------- inside `UNDERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:15:57
+    |                                                         --------------------------- inside `UNDERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:14:57
99 error[E0080]: evaluation of constant value failed
100   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


105    |                  pointer arithmetic failed: allocN has size 1, so pointer to 2 bytes starting at offset -4 is out-of-bounds
106    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:16:49
+   ::: $DIR/offset_ub.rs:15:49
109    |
109    |
110 LL | pub const NEGATIVE_OFFSET: *const u8 = unsafe { [0u8; 1].as_ptr().wrapping_offset(-2).offset(-2) };
-    |                                                 ------------------------------------------------ inside `NEGATIVE_OFFSET` at $DIR/offset_ub.rs:16:49
+    |                                                 ------------------------------------------------ inside `NEGATIVE_OFFSET` at $DIR/offset_ub.rs:15:49
113 error[E0080]: evaluation of constant value failed
114   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


119    |                  pointer arithmetic failed: allocN has size 0, so pointer to 1 byte starting at offset 0 is out-of-bounds
120    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:18:50
+   ::: $DIR/offset_ub.rs:17:50
123    |
123    |
124 LL | pub const ZERO_SIZED_ALLOC: *const u8 = unsafe { [0u8; 0].as_ptr().offset(1) };
-    |                                                  --------------------------- inside `ZERO_SIZED_ALLOC` at $DIR/offset_ub.rs:18:50
+    |                                                  --------------------------- inside `ZERO_SIZED_ALLOC` at $DIR/offset_ub.rs:17:50
127 error[E0080]: evaluation of constant value failed
128   --> $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL

133    |                  0x1 is not a valid pointer
133    |                  0x1 is not a valid pointer
134    |                  inside `ptr::mut_ptr::<impl *mut u8>::offset` at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:19:42
+   ::: $DIR/offset_ub.rs:18:42
137    |
137    |
138 LL | pub const DANGLING: *const u8 = unsafe { ptr::NonNull::<u8>::dangling().as_ptr().offset(4) };
-    |                                          ------------------------------------------------- inside `DANGLING` at $DIR/offset_ub.rs:19:42
+    |                                          ------------------------------------------------- inside `DANGLING` at $DIR/offset_ub.rs:18:42
141 error[E0080]: evaluation of constant value failed
142   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

147    |                  pointer arithmetic failed: 0x0 is not a valid pointer
147    |                  pointer arithmetic failed: 0x0 is not a valid pointer
148    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:22:50
+   ::: $DIR/offset_ub.rs:21:50
151    |
151    |
152 LL | pub const NULL_OFFSET_ZERO: *const u8 = unsafe { ptr::null::<u8>().offset(0) };
-    |                                                  --------------------------- inside `NULL_OFFSET_ZERO` at $DIR/offset_ub.rs:22:50
+    |                                                  --------------------------- inside `NULL_OFFSET_ZERO` at $DIR/offset_ub.rs:21:50
155 error[E0080]: evaluation of constant value failed
156   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL


161    |                  0x7f..f is not a valid pointer
162    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_ub.rs:25:47
+   ::: $DIR/offset_ub.rs:24:47
165    |
165    |
166 LL | pub const UNDERFLOW_ABS: *const u8 = unsafe { (usize::MAX as *const u8).offset(isize::MIN) };
-    |                                               -------------------------------------------- inside `UNDERFLOW_ABS` at $DIR/offset_ub.rs:25:47
+    |                                               -------------------------------------------- inside `UNDERFLOW_ABS` at $DIR/offset_ub.rs:24:47
169 error: aborting due to 12 previous errors
170 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/offset_ub.stderr
To only update this specific test, also pass `--test-args consts/offset_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:7:46
   |
   |
LL | pub const BEFORE_START: *const u8 = unsafe { (&0u8 as *const u8).offset(-1) }; //~NOTE
   |                                              ------------------------------ inside `BEFORE_START` at /checkout/src/test/ui/consts/offset_ub.rs:7:46
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc6 has size 1, so pointer to 2 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:8:43
   |
   |
LL | pub const AFTER_END: *const u8 = unsafe { (&0u8 as *const u8).offset(2) }; //~NOTE
   |                                           ----------------------------- inside `AFTER_END` at /checkout/src/test/ui/consts/offset_ub.rs:8:43
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc9 has size 100, so pointer to 101 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:9:45
   |
   |
LL | pub const AFTER_ARRAY: *const u8 = unsafe { [0u8; 100].as_ptr().offset(101) }; //~NOTE
   |                                             ------------------------------- inside `AFTER_ARRAY` at /checkout/src/test/ui/consts/offset_ub.rs:9:45
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u16>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:11:43
   |
   |
LL | pub const OVERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MAX) }; //~NOTE
   |                                           ------------------------------------- inside `OVERFLOW` at /checkout/src/test/ui/consts/offset_ub.rs:11:43
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u16>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:12:44
   |
   |
LL | pub const UNDERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MIN) }; //~NOTE
   |                                            ------------------------------------- inside `UNDERFLOW` at /checkout/src/test/ui/consts/offset_ub.rs:12:44
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:13:56
   |
   |
LL | pub const OVERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (usize::MAX as *const u8).offset(2) }; //~NOTE
   |                                                        ----------------------------------- inside `OVERFLOW_ADDRESS_SPACE` at /checkout/src/test/ui/consts/offset_ub.rs:13:56
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:14:57
   |
   |
LL | pub const UNDERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (1 as *const u8).offset(-2) }; //~NOTE
   |                                                         --------------------------- inside `UNDERFLOW_ADDRESS_SPACE` at /checkout/src/test/ui/consts/offset_ub.rs:14:57
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc24 has size 1, so pointer to 2 bytes starting at offset -4 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:15:49
   |
   |
LL | pub const NEGATIVE_OFFSET: *const u8 = unsafe { [0u8; 1].as_ptr().wrapping_offset(-2).offset(-2) }; //~NOTE
   |                                                 ------------------------------------------------ inside `NEGATIVE_OFFSET` at /checkout/src/test/ui/consts/offset_ub.rs:15:49
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc27 has size 0, so pointer to 1 byte starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:17:50
   |
   |
LL | pub const ZERO_SIZED_ALLOC: *const u8 = unsafe { [0u8; 0].as_ptr().offset(1) }; //~NOTE
   |                                                  --------------------------- inside `ZERO_SIZED_ALLOC` at /checkout/src/test/ui/consts/offset_ub.rs:17:50
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mut_ptr.rs:307:18
   |
LL |         unsafe { intrinsics::offset(self, count) as *mut T }
LL |         unsafe { intrinsics::offset(self, count) as *mut T }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  0x1 is not a valid pointer
   |                  inside `ptr::mut_ptr::<impl *mut u8>::offset` at /checkout/library/core/src/ptr/mut_ptr.rs:307:18
   |
  ::: /checkout/src/test/ui/consts/offset_ub.rs:18:42
   |
LL | pub const DANGLING: *const u8 = unsafe { ptr::NonNull::<u8>::dangling().as_ptr().offset(4) }; //~NOTE
   |                                          ------------------------------------------------- inside `DANGLING` at /checkout/src/test/ui/consts/offset_ub.rs:18:42
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: 0x0 is not a valid pointer
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:21:50
   |
   |
LL | pub const NULL_OFFSET_ZERO: *const u8 = unsafe { ptr::null::<u8>().offset(0) }; //~NOTE
   |                                                  --------------------------- inside `NULL_OFFSET_ZERO` at /checkout/src/test/ui/consts/offset_ub.rs:21:50
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  0x7fffffffffffffff is not a valid pointer
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:24:47
   |
   |
LL | pub const UNDERFLOW_ABS: *const u8 = unsafe { (usize::MAX as *const u8).offset(isize::MIN) }; //~NOTE
   |                                               -------------------------------------------- inside `UNDERFLOW_ABS` at /checkout/src/test/ui/consts/offset_ub.rs:24:47
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0080`.


------------------------------------------


---- [ui] ui/consts/ptr_comparisons.rs stdout ----
diff of stderr:

7    |                  pointer arithmetic failed: alloc3 has size $WORD, so pointer to $TWO_WORDS bytes starting at offset 0 is out-of-bounds
8    |                  inside `ptr::const_ptr::<impl *const usize>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/ptr_comparisons.rs:59:34
+   ::: $DIR/ptr_comparisons.rs:58:34
11    |
11    |
12 LL | const _: *const usize = unsafe { (FOO as *const usize).offset(2) };
-    |                                  ------------------------------- inside `_` at $DIR/ptr_comparisons.rs:59:34
+    |                                  ------------------------------- inside `_` at $DIR/ptr_comparisons.rs:58:34
15 error[E0080]: evaluation of constant value failed
-   --> $DIR/ptr_comparisons.rs:62:33
+   --> $DIR/ptr_comparisons.rs:61:33
17    |
17    |
18 LL |     unsafe { std::ptr::addr_of!((*(FOO as *const usize as *const [u8; 1000]))[999]) };
19    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: alloc3 has size $WORD, so pointer to 1000 bytes starting at offset 0 is out-of-bounds
20 
21 error: any use of this value will cause an error
-   --> $DIR/ptr_comparisons.rs:66:27
+   --> $DIR/ptr_comparisons.rs:65:27
+   --> $DIR/ptr_comparisons.rs:65:27
23    |
24 LL | const _: usize = unsafe { std::mem::transmute::<*const usize, usize>(FOO) + 4 };

31    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
32 
33 error: any use of this value will cause an error
33 error: any use of this value will cause an error
-   --> $DIR/ptr_comparisons.rs:71:27
+   --> $DIR/ptr_comparisons.rs:70:27
35    |
36 LL | const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/ptr_comparisons.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/ptr_comparisons.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/ptr_comparisons.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/ptr_comparisons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:295:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: alloc3 has size 8, so pointer to 16 bytes starting at offset 0 is out-of-bounds
   |                  inside `ptr::const_ptr::<impl *const usize>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:295:18
  ::: /checkout/src/test/ui/consts/ptr_comparisons.rs:58:34
   |
   |
LL | const _: *const usize = unsafe { (FOO as *const usize).offset(2) };
   |                                  ------------------------------- inside `_` at /checkout/src/test/ui/consts/ptr_comparisons.rs:58:34
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:61:33
   |
   |
LL |     unsafe { std::ptr::addr_of!((*(FOO as *const usize as *const [u8; 1000]))[999]) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: alloc3 has size 8, so pointer to 1000 bytes starting at offset 0 is out-of-bounds
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:65:27
   |
   |
LL | const _: usize = unsafe { std::mem::transmute::<*const usize, usize>(FOO) + 4 };
   |                           |
   |                           |
   |                           unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:70:27
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:70:27
   |
LL | const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };
   |                           |
   |                           |
