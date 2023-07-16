plain
................F....................................................................... 2552/12950
........................................................................................ 2640/12950
........................................................................................ 2728/12950
........................................................................................ 2816/12950
.........................F.......F..F.............................i..................... 2904/12950
........................................................................................ 3080/12950
.................................................................................iiiii.. 3168/12950
........................................................................................ 3256/12950
........................................................................................ 3344/12950
---
To only update this specific test, also pass `--test-args consts/const-eval/issue-91827-extern-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-91827-extern-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-91827-extern-types/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-91827-extern-types/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
warning: the feature `const_ptr_offset_from` has been stable since 1.62.0 and no longer requires an attribute to enable
   |
LL | #![feature(const_ptr_offset_from)]
   |            ^^^^^^^^^^^^^^^^^^^^^
   |
---
1 error[E0080]: evaluation of constant value failed
2   --> $DIR/offset_from_ub.rs:17:27
3    |

54 LL |     unsafe { ptr_offset_from(end_ptr, end_ptr) }
55    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc26 has size 4, so pointer at offset 10 is out-of-bounds
- error: aborting due to 8 previous errors
+ error: aborting due to 8 previous errors; 1 warning emitted
58 
59 For more information about this error, try `rustc --explain E0080`.
59 For more information about this error, try `rustc --explain E0080`.
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
warning: the feature `const_ptr_offset_from` has been stable since 1.62.0 and no longer requires an attribute to enable
   |
LL | #![feature(const_ptr_offset_from)]
   |            ^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(stable_features)]` on by default

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:17:27
   |
LL |     let offset = unsafe { ptr_offset_from(field_ptr, base_ptr) }; //~ERROR evaluation of constant value failed
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ptr_offset_from cannot compute offset of pointers into different allocations.
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:611:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                  |
   |                  out-of-bounds offset_from: 0x2a is not a valid pointer
   |                  out-of-bounds offset_from: 0x2a is not a valid pointer
   |                  inside `ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:611:18
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:23:14
   |
   |
LL |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
   |              ----------------------------------- inside `NOT_PTR` at /checkout/src/test/ui/consts/offset_from_ub.rs:23:14
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:30:14
   |
   |
LL |     unsafe { ptr_offset_from(field_ptr, base_ptr as *const u16) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exact_div: 1_isize cannot be divided by 2_isize without remainder
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:36:14
   |
   |
LL |     unsafe { ptr_offset_from(ptr, ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: null pointer is not a valid pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:43:14
   |
   |
LL |     unsafe { ptr_offset_from(ptr2, ptr1) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: 0x10 is not a valid pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:52:14
   |
   |
LL |     unsafe { ptr_offset_from(end_ptr, start_ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc20 has size 4, so pointer at offset 10 is out-of-bounds
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:61:14
   |
   |
LL |     unsafe { ptr_offset_from(start_ptr, end_ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc23 has size 4, so pointer at offset 10 is out-of-bounds
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:69:14
   |
   |
LL |     unsafe { ptr_offset_from(end_ptr, end_ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc26 has size 4, so pointer at offset 10 is out-of-bounds
error: aborting due to 8 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---
To only update this specific test, also pass `--test-args consts/offset_from.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `const_ptr_offset_from` has been stable since 1.62.0 and no longer requires an attribute to enable
   |
LL | #![feature(const_ptr_offset_from)]
   |            ^^^^^^^^^^^^^^^^^^^^^
   |
---
To only update this specific test, also pass `--test-args consts/offset.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `const_ptr_offset_from` has been stable since 1.62.0 and no longer requires an attribute to enable
   |
LL | #![feature(const_ptr_offset_from)]
   |            ^^^^^^^^^^^^^^^^^^^^^
   |
