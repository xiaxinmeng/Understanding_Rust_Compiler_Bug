plain
-   --> $DIR/offset_from_ub.rs:17:27
+ error[E0635]: unknown feature `const_ptr_offset`
+   --> $DIR/offset_from_ub.rs:1:35
3    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL |     let offset = unsafe { ptr_offset_from(field_ptr, base_ptr) };
-    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ptr_offset_from cannot compute offset of pointers into different allocations.
+ LL | #![feature(const_ptr_offset_from, const_ptr_offset)]
6 
- error[E0080]: evaluation of constant value failed
-   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |
-    |
- LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
-    |                  |
-    |                  out-of-bounds offset_from: 0x2a is not a valid pointer
-    |                  out-of-bounds offset_from: 0x2a is not a valid pointer
-    |                  inside `ptr::const_ptr::<impl *const u8>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_from_ub.rs:23:14
-    |
-    |
- LL |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
-    |              ----------------------------------- inside `NOT_PTR` at $DIR/offset_from_ub.rs:23:14
+ error: aborting due to previous error
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:30:14
-    |
-    |
- LL |     unsafe { ptr_offset_from(field_ptr, base_ptr as *const u16) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exact_div: 1_isize cannot be divided by 2_isize without remainder
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:36:14
-    |
- LL |     unsafe { ptr_offset_from(ptr, ptr) }
- LL |     unsafe { ptr_offset_from(ptr, ptr) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: null pointer is not a valid pointer
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:43:14
-    |
- LL |     unsafe { ptr_offset_from(ptr2, ptr1) }
- LL |     unsafe { ptr_offset_from(ptr2, ptr1) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: 0x10 is not a valid pointer
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:52:14
-    |
-    |
- LL |     unsafe { ptr_offset_from(end_ptr, start_ptr) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc18 has size 4, so pointer at offset 10 is out-of-bounds
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:61:14
-    |
-    |
- LL |     unsafe { ptr_offset_from(start_ptr, end_ptr) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc21 has size 4, so pointer at offset 10 is out-of-bounds
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:69:14
-    |
-    |
- LL |     unsafe { ptr_offset_from(end_ptr, end_ptr) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc24 has size 4, so pointer at offset 10 is out-of-bounds
- error: aborting due to 8 previous errors
- 
- For more information about this error, try `rustc --explain E0080`.
+ For more information about this error, try `rustc --explain E0635`.
+ For more information about this error, try `rustc --explain E0635`.
60 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:1:35
   |
LL | #![feature(const_ptr_offset_from, const_ptr_offset)]
   |                                   ^^^^^^^^^^^^^^^^
