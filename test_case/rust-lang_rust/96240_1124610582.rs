plain

---- [ui] src/test/ui/consts/offset_from_ub.rs stdout ----
diff of stderr:

- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:16:27
+ error: `ptr_offset_from_unsigned` is not yet stable as a const fn
3    |
3    |
- LL |     let offset = unsafe { ptr_offset_from(field_ptr, base_ptr) };
-    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ptr_offset_from cannot compute offset of pointers into different allocations.
- error[E0080]: evaluation of constant value failed
-   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+ LL |     let offset = unsafe { ptr_offset_from_unsigned(field_ptr, base_ptr) };
9    |
9    |
- LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
-    |                  |
-    |                  out-of-bounds offset_from: 0x2a is not a valid pointer
-    |                  out-of-bounds offset_from: 0x2a is not a valid pointer
-    |                  inside `ptr::const_ptr::<impl *const u8>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_from_ub.rs:22:14
-    |
-    |
- LL |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |              ----------------------------------- inside `NOT_PTR` at $DIR/offset_from_ub.rs:22:14
+    = help: add `#![feature(const_ptr_sub_ptr)]` to the crate attributes to enable
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:29:14
-   --> $DIR/offset_from_ub.rs:29:14
+ error: `ptr_offset_from_unsigned` is not yet stable as a const fn
23    |
23    |
- LL |     unsafe { ptr_offset_from(field_ptr, base_ptr as *const u16) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exact_div: 1_isize cannot be divided by 2_isize without remainder
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:35:14
-   --> $DIR/offset_from_ub.rs:35:14
+ LL |     unsafe { ptr_offset_from_unsigned(p, p.add(2) ) }
29    |
- LL |     unsafe { ptr_offset_from(ptr, ptr) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: null pointer is not a valid pointer
+    = help: add `#![feature(const_ptr_sub_ptr)]` to the crate attributes to enable
+    = help: add `#![feature(const_ptr_sub_ptr)]` to the crate attributes to enable
32 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:42:14
-    |
- LL |     unsafe { ptr_offset_from(ptr2, ptr1) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: 0x10 is not a valid pointer
+ error: aborting due to 2 previous errors
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:51:14
-    |
-    |
- LL |     unsafe { ptr_offset_from(end_ptr, start_ptr) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc20 has size 4, so pointer at offset 10 is out-of-bounds
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:60:14
-    |
-    |
- LL |     unsafe { ptr_offset_from(start_ptr, end_ptr) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc23 has size 4, so pointer at offset 10 is out-of-bounds
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:68:14
-    |
-    |
- LL |     unsafe { ptr_offset_from(end_ptr, end_ptr) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc26 has size 4, so pointer at offset 10 is out-of-bounds
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:78:27
-    |
-    |
- LL |     let offset = unsafe { ptr_offset_from_unsigned(field_ptr, base_ptr) };
-    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ptr_offset_from_unsigned cannot compute offset of pointers into different allocations.
- error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:86:14
-    |
-    |
- LL |     unsafe { ptr_offset_from_unsigned(p, p.add(2) ) }
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ptr_offset_from_unsigned cannot compute a negative offset, but 0 < 8
- error: aborting due to 10 previous errors
- 
- For more information about this error, try `rustc --explain E0080`.
72 
72 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary"
stdout: none
--- stderr -------------------------------
error: `ptr_offset_from_unsigned` is not yet stable as a const fn
   |
   |
LL |     let offset = unsafe { ptr_offset_from_unsigned(field_ptr, base_ptr) }; //~ERROR evaluation of constant value failed
   |
   = help: add `#![feature(const_ptr_sub_ptr)]` to the crate attributes to enable


error: `ptr_offset_from_unsigned` is not yet stable as a const fn
   |
   |
LL |     unsafe { ptr_offset_from_unsigned(p, p.add(2) ) } //~ERROR evaluation of constant value failed
   |
   = help: add `#![feature(const_ptr_sub_ptr)]` to the crate attributes to enable

error: aborting due to 2 previous errors
