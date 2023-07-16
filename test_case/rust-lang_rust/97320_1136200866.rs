plain

---- [ui] src/test/ui/const-ptr/out_of_bounds_read.rs stdout ----
diff of stderr:

7    |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
8    |         inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-   ::: $DIR/out_of_bounds_read.rs:12:33
+   ::: $DIR/out_of_bounds_read.rs:10:33
11    |
11    |
12 LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |                                 ----------------------- inside `_READ` at $DIR/out_of_bounds_read.rs:12:33
+    |                                 ----------------------- inside `_READ` at $DIR/out_of_bounds_read.rs:10:33
15 error[E0080]: evaluation of constant value failed
16   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL


26 LL |         unsafe { read(self) }
27    |                  ---------- inside `ptr::const_ptr::<impl *const u32>::read` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/out_of_bounds_read.rs:13:39
+   ::: $DIR/out_of_bounds_read.rs:11:39
30    |
30    |
31 LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };
-    |                                       ------------------- inside `_CONST_READ` at $DIR/out_of_bounds_read.rs:13:39
+    |                                       ------------------- inside `_CONST_READ` at $DIR/out_of_bounds_read.rs:11:39
34 error[E0080]: evaluation of constant value failed
35   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL


45 LL |         unsafe { read(self) }
46    |                  ---------- inside `ptr::mut_ptr::<impl *mut u32>::read` at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-   ::: $DIR/out_of_bounds_read.rs:14:37
+   ::: $DIR/out_of_bounds_read.rs:12:37
49    |
49    |
50 LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };
-    |                                     --------------------------------- inside `_MUT_READ` at $DIR/out_of_bounds_read.rs:14:37
+    |                                     --------------------------------- inside `_MUT_READ` at $DIR/out_of_bounds_read.rs:12:37
53 error: aborting due to 3 previous errors
54 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/out_of_bounds_read.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-ptr/out_of_bounds_read.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-ptr/out_of_bounds_read.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         |
   |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:1109:9
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:10:33
   |
   |
LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
   |                                 ----------------------- inside `_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:10:33
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mod.rs:1109:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:1109:9
  ::: /checkout/library/core/src/ptr/const_ptr.rs:1150:18
   |
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::const_ptr::<impl *const u32>::read` at /checkout/library/core/src/ptr/const_ptr.rs:1150:18
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:11:39
   |
   |
LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };
   |                                       ------------------- inside `_CONST_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:11:39
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mod.rs:1109:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:1109:9
  ::: /checkout/library/core/src/ptr/mut_ptr.rs:1262:18
   |
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::mut_ptr::<impl *mut u32>::read` at /checkout/library/core/src/ptr/mut_ptr.rs:1262:18
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:12:37
   |
   |
LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };
   |                                     --------------------------------- inside `_MUT_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:12:37
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/issue-miri-1910.rs stdout ----
diff of stderr:

7    |           unable to turn pointer into raw bytes
8    |           inside `std::ptr::read::<u8>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
9    |           inside `ptr::const_ptr::<impl *const u8>::read` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |           inside `C` at $DIR/issue-miri-1910.rs:7:5
+    |           inside `C` at $DIR/issue-miri-1910.rs:6:5
-   ::: $DIR/issue-miri-1910.rs:4:1
+   ::: $DIR/issue-miri-1910.rs:3:1
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
stdout: none
--- stderr -------------------------------
error: any use of this value will cause an error
   |
LL |           copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |           |
   |           |
   |           unable to turn pointer into raw bytes
   |           inside `std::ptr::read::<u8>` at /checkout/library/core/src/ptr/mod.rs:1109:9
   |           inside `ptr::const_ptr::<impl *const u8>::read` at /checkout/library/core/src/ptr/const_ptr.rs:1150:18
   |           inside `C` at /checkout/src/test/ui/consts/issue-miri-1910.rs:6:5
  ::: /checkout/src/test/ui/consts/issue-miri-1910.rs:3:1
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
