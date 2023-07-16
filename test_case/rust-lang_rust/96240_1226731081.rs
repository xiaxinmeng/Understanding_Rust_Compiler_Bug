plain
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:17:27
+   --> $DIR/offset_from_ub.rs:18:27
3    |
4 LL |     let offset = unsafe { ptr_offset_from(field_ptr, base_ptr) };
5    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from` called on pointers into different allocations

13    |                  `ptr_offset_from` called on pointers into different allocations
14    |                  inside `ptr::const_ptr::<impl *const u8>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_from_ub.rs:23:14
+   ::: $DIR/offset_from_ub.rs:24:14
17    |
17    |
18 LL |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
-    |              ----------------------------------- inside `NOT_PTR` at $DIR/offset_from_ub.rs:23:14
+    |              ----------------------------------- inside `NOT_PTR` at $DIR/offset_from_ub.rs:24:14
21 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:30:14
+   --> $DIR/offset_from_ub.rs:31:14
23    |
23    |
24 LL |     unsafe { ptr_offset_from(field_ptr, base_ptr as *const u16) }
25    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exact_div: 1_isize cannot be divided by 2_isize without remainder
26 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
27 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:36:14
---
33 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:43:14
+   --> $DIR/offset_from_ub.rs:44:14
35    |
36 LL |     unsafe { ptr_offset_from(ptr2, ptr1) }
37    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: 0x8[noalloc] is a dangling pointer (it has no provenance)
38 
39 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:52:14
+   --> $DIR/offset_from_ub.rs:53:14
+   --> $DIR/offset_from_ub.rs:53:14
41    |
42 LL |     unsafe { ptr_offset_from(end_ptr, start_ptr) }
43    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc18 has size 4, so pointer to 10 bytes starting at offset 0 is out-of-bounds
44 
45 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:61:14
+   --> $DIR/offset_from_ub.rs:62:14
+   --> $DIR/offset_from_ub.rs:62:14
47    |
48 LL |     unsafe { ptr_offset_from(start_ptr, end_ptr) }
49    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc21 has size 4, so pointer to 10 bytes starting at offset 0 is out-of-bounds
50 
51 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:69:14
+   --> $DIR/offset_from_ub.rs:70:14
+   --> $DIR/offset_from_ub.rs:70:14
53    |
54 LL |     unsafe { ptr_offset_from(end_ptr, end_ptr) }
55    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc24 has size 4, so pointer at offset 10 is out-of-bounds
56 
57 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:78:14
+   --> $DIR/offset_from_ub.rs:79:14
+   --> $DIR/offset_from_ub.rs:79:14
59    |
60 LL |     unsafe { ptr_offset_from_unsigned(field_ptr, base_ptr) }
61    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called on pointers into different allocations
62 
63 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:85:14
+   --> $DIR/offset_from_ub.rs:86:14
+   --> $DIR/offset_from_ub.rs:86:14
65    |
66 LL |     unsafe { ptr_offset_from(ptr2, ptr1) }
67    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from` called when first pointer is too far ahead of second
68 
69 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:91:14
+   --> $DIR/offset_from_ub.rs:92:14
+   --> $DIR/offset_from_ub.rs:92:14
71    |
72 LL |     unsafe { ptr_offset_from(ptr1, ptr2) }
73    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from` called when first pointer is too far before second
74 
75 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:98:14
+   --> $DIR/offset_from_ub.rs:99:14
+   --> $DIR/offset_from_ub.rs:99:14
77    |
78 LL |     unsafe { ptr_offset_from_unsigned(p, p.add(2) ) }
79    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called when first pointer has smaller offset than second: 0 < 8
80 
81 error[E0080]: evaluation of constant value failed
-   --> $DIR/offset_from_ub.rs:105:14
+   --> $DIR/offset_from_ub.rs:106:14
+   --> $DIR/offset_from_ub.rs:106:14
83    |
84 LL |     unsafe { ptr_offset_from_unsigned(ptr2, ptr1) }
85    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called when first pointer is too far ahead of second
93    |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
93    |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
94    |                  inside `ptr::const_ptr::<impl *const u8>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_from_ub.rs:114:14
+   ::: $DIR/offset_from_ub.rs:115:14
97    |
98 LL |     unsafe { ptr2.offset_from(ptr1) }
98 LL |     unsafe { ptr2.offset_from(ptr1) }
-    |              ---------------------- inside `OFFSET_VERY_FAR1` at $DIR/offset_from_ub.rs:114:14
+    |              ---------------------- inside `OFFSET_VERY_FAR1` at $DIR/offset_from_ub.rs:115:14
101 error[E0080]: evaluation of constant value failed
102   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

107    |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
107    |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
108    |                  inside `ptr::const_ptr::<impl *const u8>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-   ::: $DIR/offset_from_ub.rs:120:14
+   ::: $DIR/offset_from_ub.rs:121:14
111    |
111    |
112 LL |     unsafe { ptr1.offset_from(ptr2.wrapping_offset(1)) }
-    |              ----------------------------------------- inside `OFFSET_VERY_FAR2` at $DIR/offset_from_ub.rs:120:14
+    |              ----------------------------------------- inside `OFFSET_VERY_FAR2` at $DIR/offset_from_ub.rs:121:14
115 error: aborting due to 15 previous errors
116 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:18:27
   |
   |
LL |     let offset = unsafe { ptr_offset_from(field_ptr, base_ptr) }; //~ERROR evaluation of constant value failed
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from` called on pointers into different allocations
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:654:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                  |
   |                  |
   |                  `ptr_offset_from` called on pointers into different allocations
   |                  inside `ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:654:18
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:24:14
   |
   |
LL |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
   |              ----------------------------------- inside `NOT_PTR` at /checkout/src/test/ui/consts/offset_from_ub.rs:24:14
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:31:14
   |
   |
LL |     unsafe { ptr_offset_from(field_ptr, base_ptr as *const u16) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exact_div: 1_isize cannot be divided by 2_isize without remainder
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:37:14
   |
   |
LL |     unsafe { ptr_offset_from(ptr, ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:44:14
   |
   |
LL |     unsafe { ptr_offset_from(ptr2, ptr1) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: 0x8[noalloc] is a dangling pointer (it has no provenance)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:53:14
   |
   |
LL |     unsafe { ptr_offset_from(end_ptr, start_ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc18 has size 4, so pointer to 10 bytes starting at offset 0 is out-of-bounds
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:62:14
   |
   |
LL |     unsafe { ptr_offset_from(start_ptr, end_ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc21 has size 4, so pointer to 10 bytes starting at offset 0 is out-of-bounds
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:70:14
   |
   |
LL |     unsafe { ptr_offset_from(end_ptr, end_ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds offset_from: alloc24 has size 4, so pointer at offset 10 is out-of-bounds
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:79:14
   |
   |
LL |     unsafe { ptr_offset_from_unsigned(field_ptr, base_ptr) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called on pointers into different allocations
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:86:14
   |
   |
LL |     unsafe { ptr_offset_from(ptr2, ptr1) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from` called when first pointer is too far ahead of second
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:92:14
   |
   |
LL |     unsafe { ptr_offset_from(ptr1, ptr2) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from` called when first pointer is too far before second
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:99:14
   |
   |
LL |     unsafe { ptr_offset_from_unsigned(p, p.add(2) ) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called when first pointer has smaller offset than second: 0 < 8
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/offset_from_ub.rs:106:14
   |
   |
LL |     unsafe { ptr_offset_from_unsigned(ptr2, ptr1) } //~ERROR evaluation of constant value failed
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ptr_offset_from_unsigned` called when first pointer is too far ahead of second
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:654:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                  |
   |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |                  inside `ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:654:18
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:115:14
   |
LL |     unsafe { ptr2.offset_from(ptr1) }
LL |     unsafe { ptr2.offset_from(ptr1) }
   |              ---------------------- inside `OFFSET_VERY_FAR1` at /checkout/src/test/ui/consts/offset_from_ub.rs:115:14
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:654:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                  |
   |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |                  out-of-bounds offset_from: null pointer is a dangling pointer (it has no provenance)
   |                  inside `ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:654:18
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:121:14
   |
   |
LL |     unsafe { ptr1.offset_from(ptr2.wrapping_offset(1)) }
   |              ----------------------------------------- inside `OFFSET_VERY_FAR2` at /checkout/src/test/ui/consts/offset_from_ub.rs:121:14
error: aborting due to 15 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
