plain
........................................................................................ 10648/13208
.............i.......................................................................... 10736/13208
.......................iiiiii.i..iiiiii.i............................................... 10824/13208
........................................................................................ 10912/13208
...................F.....F......F.......F............................................... 11000/13208
........................................................................................ 11176/13208
........................................................................................ 11264/13208
........................................................................................ 11352/13208
........................................................................................ 11440/13208
---
diff of stderr:

2   --> $DIR/match_arr_unknown_len.rs:3:9
3    |
4 LL |         [1, 2] => true,
-    |         ^^^^^^ expected `2_usize`, found `N`
+    |         ^^^^^^ expected `2`, found `N`
6    |
7    = note: expected array `[u32; 2]`
8               found array `[u32; N]`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/match_arr_unknown_len/match_arr_unknown_len.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args array-slice-vec/match_arr_unknown_len.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/match_arr_unknown_len.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/match_arr_unknown_len" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/match_arr_unknown_len/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/array-slice-vec/match_arr_unknown_len.rs:3:9
   |
   |
LL |         [1, 2] => true, //~ ERROR mismatched types
   |         ^^^^^^ expected `2`, found `N`
   |
   = note: expected array `[u32; 2]`
              found array `[u32; N]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/issue-85155.rs stdout ----
diff of stderr:

- error[E0080]: evaluation of `post_monomorphization_error::ValidateConstImm::<2_i32, 0_i32, 1_i32>::VALID` failed
+ error[E0080]: evaluation of `post_monomorphization_error::ValidateConstImm::<2, 0, 1>::VALID` failed
2   --> $DIR/auxiliary/post_monomorphization_error.rs:7:17
3    |
4 LL |         let _ = 1 / ((IMM >= MIN && IMM <= MAX) as usize);

5    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to divide `1_usize` by zero
6 
- note: the above error was encountered while instantiating `fn post_monomorphization_error::stdarch_intrinsic::<2_i32>`
+ note: the above error was encountered while instantiating `fn post_monomorphization_error::stdarch_intrinsic::<2>`
9    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
10 LL |     post_monomorphization_error::stdarch_intrinsic::<2>();

---
To only update this specific test, also pass `--test-args consts/const-eval/issue-85155.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-85155.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-85155" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-85155/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `post_monomorphization_error::ValidateConstImm::<2, 0, 1>::VALID` failed
  --> /checkout/src/test/ui/consts/const-eval/auxiliary/post_monomorphization_error.rs:7:17
   |
LL |         let _ = 1 / ((IMM >= MIN && IMM <= MAX) as usize);
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to divide `1_usize` by zero

note: the above error was encountered while instantiating `fn post_monomorphization_error::stdarch_intrinsic::<2>`
   |
LL |     post_monomorphization_error::stdarch_intrinsic::<2>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

---

---- [ui] src/test/ui/dropck/reject-specialized-drops-8142.rs stdout ----
diff of stderr:

104 LL | impl              Drop for X<3>           { fn drop(&mut self) { } } // REJECT
106    |
106    |
-    = note: `3_usize` is not a generic parameter
+    = note: `3` is not a generic parameter
108 note: use the same sequence of generic lifetime, type and const parameters as the struct definition
109   --> $DIR/reject-specialized-drops-8142.rs:17:1


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/reject-specialized-drops-8142/reject-specialized-drops-8142.stderr
To only update this specific test, also pass `--test-args dropck/reject-specialized-drops-8142.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/reject-specialized-drops-8142" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/reject-specialized-drops-8142/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0367]: `Drop` impl requires `'adds_bnd: 'al` but the struct it is implemented for does not
   |
   |
LL | impl<'al,'adds_bnd:'al> Drop for K<'al,'adds_bnd> {                        // REJECT
   |
note: the implementor must specify the same requirement
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:4:1
   |
   |
LL | struct K<'l1,'l2> { x: &'l1 i8, y: &'l2 u8 }


error[E0367]: `Drop` impl requires `'adds_bnd: 'al` but the struct it is implemented for does not
   |
   |
LL | impl<'al,'adds_bnd>     Drop for L<'al,'adds_bnd> where 'adds_bnd:'al {    // REJECT
   |
note: the implementor must specify the same requirement
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:5:1
   |
   |
LL | struct L<'l1,'l2> { x: &'l1 i8, y: &'l2 u8 }

error[E0366]: `Drop` impls cannot be specialized
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:34:1
   |
   |
LL | impl                    Drop for N<'static>     { fn drop(&mut self) { } } // REJECT
   |
   |
   = note: `'static` is not a generic parameter
note: use the same sequence of generic lifetime, type and const parameters as the struct definition
   |
   |
LL | struct N<'n> { x: &'n i8 }

error[E0366]: `Drop` impls cannot be specialized
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:39:1
   |
   |
LL | impl              Drop for P<i8>          { fn drop(&mut self) { } } // REJECT
   |
   |
   = note: `i8` is not a generic parameter
note: use the same sequence of generic lifetime, type and const parameters as the struct definition
   |
   |
LL | struct P<Tp> { x: *const Tp }


error[E0367]: `Drop` impl requires `AddsBnd: Bound` but the struct it is implemented for does not
   |
   |
LL | impl<AddsBnd:Bound> Drop for Q<AddsBnd> { fn drop(&mut self) { } } // REJECT
   |
note: the implementor must specify the same requirement
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:10:1
   |
   |
LL | struct Q<Tq> { x: *const Tq }


error[E0367]: `Drop` impl requires `AddsRBnd: 'rbnd` but the struct it is implemented for does not
   |
   |
LL | impl<'rbnd,AddsRBnd:'rbnd> Drop for R<AddsRBnd> { fn drop(&mut self) { } } // REJECT
   |
note: the implementor must specify the same requirement
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:11:1
   |
   |
LL | struct R<Tr> { x: *const Tr }

error[E0366]: `Drop` impls cannot be specialized
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:54:1
   |
   |
LL | impl<One>         Drop for V<One,One>     { fn drop(&mut self) { } } // REJECT
   |
   = note: `One` is mentioned multiple times
note: use the same sequence of generic lifetime, type and const parameters as the struct definition
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:15:1
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:15:1
   |
LL | struct V<Tva, Tvb> { x: *const Tva, y: *const Tvb }

error[E0366]: `Drop` impls cannot be specialized
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:57:1
   |
   |
LL | impl<'lw>         Drop for W<'lw,'lw>     { fn drop(&mut self) { } } // REJECT
   |
   |
   = note: `'lw` is mentioned multiple times
note: use the same sequence of generic lifetime, type and const parameters as the struct definition
   |
   |
LL | struct W<'l1, 'l2> { x: &'l1 i8, y: &'l2 u8 }

error[E0366]: `Drop` impls cannot be specialized
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:60:1
   |
   |
LL | impl              Drop for X<3>           { fn drop(&mut self) { } } // REJECT
   |
   |
   = note: `3` is not a generic parameter
note: use the same sequence of generic lifetime, type and const parameters as the struct definition
   |
   |
LL | struct X<const Ca: usize>;

error[E0366]: `Drop` impls cannot be specialized
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:63:1
   |
   |
LL | impl<const Ca: usize> Drop for Y<Ca, Ca>     { fn drop(&mut self) { } } // REJECT
   |
   = note: `Ca` is mentioned multiple times
note: use the same sequence of generic lifetime, type and const parameters as the struct definition
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:18:1
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:18:1
   |
LL | struct Y<const Ca: usize, const Cb: usize>;


error[E0367]: `Drop` impl requires `AddsBnd: Bound` but the enum it is implemented for does not
   |
   |
LL | impl<AddsBnd:Bound> Drop for Enum<AddsBnd> { fn drop(&mut self) { } } // REJECT
   |
note: the implementor must specify the same requirement
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:20:1
   |
   |
LL | enum Enum<T> { Variant(T) }


error[E0367]: `Drop` impl requires `AddsBnd: Bound` but the struct it is implemented for does not
   |
   |
LL | impl<AddsBnd:Bound> Drop for TupleStruct<AddsBnd> { fn drop(&mut self) { } } // REJECT
   |
note: the implementor must specify the same requirement
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:21:1
   |
   |
LL | struct TupleStruct<T>(T);


error[E0367]: `Drop` impl requires `AddsBnd: Bound` but the union it is implemented for does not
   |
   |
LL | impl<AddsBnd:Copy + Bound> Drop for Union<AddsBnd> { fn drop(&mut self) { } } // REJECT
   |
note: the implementor must specify the same requirement
  --> /checkout/src/test/ui/dropck/reject-specialized-drops-8142.rs:22:1
   |
   |
LL | union Union<T: Copy> { f: T }

error: aborting due to 13 previous errors

Some errors have detailed explanations: E0366, E0367.
---

12 LL |     foo::<i32>();
13    |     ^^^^^^^^^^^^
14 
- error[E0080]: evaluation of `bar::<0_usize>::{constant#0}` failed
+ error[E0080]: evaluation of `bar::<0>::{constant#0}` failed
17    |
17    |
18 LL |     const { N - 1 }
19    |             ^^^^^ attempt to compute `0_usize - 1_usize`, which would overflow
20 
20 
- note: the above error was encountered while instantiating `fn bar::<0_usize>`
+ note: the above error was encountered while instantiating `fn bar::<0>`
23    |
23    |
24 LL |     bar::<0>();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-generic-err/const-expr-generic-err.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inline-const/const-expr-generic-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inline-const/const-expr-generic-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-generic-err" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-generic-err/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `foo::<i32>::{constant#0}` failed
   |
   |
LL |     const { assert!(std::mem::size_of::<T>() == 0); } //~ ERROR E0080
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: std::mem::size_of::<T>() == 0', /checkout/src/test/ui/inline-const/const-expr-generic-err.rs:5:13
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)


note: the above error was encountered while instantiating `fn foo::<i32>`
   |
LL |     foo::<i32>();
   |     ^^^^^^^^^^^^


error[E0080]: evaluation of `bar::<0>::{constant#0}` failed
   |
   |
LL |     const { N - 1 } //~ ERROR E0080
   |             ^^^^^ attempt to compute `0_usize - 1_usize`, which would overflow

note: the above error was encountered while instantiating `fn bar::<0>`
   |
   |
LL |     bar::<0>();

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0080`.
---
diff of stderr:

2   --> $DIR/issue-98299.rs:4:5
3    |
4 LL |     SmallCString::try_from(p).map(|cstr| cstr);
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type for enum `Result<SmallCString<{_: usize}>, ()>`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type for enum `Result<SmallCString<_>, ()>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-98299/issue-98299.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-98299.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-98299.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-98299" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-98299/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-98299.rs:4:5
   |
   |
LL |     SmallCString::try_from(p).map(|cstr| cstr);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type for enum `Result<SmallCString<_>, ()>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lint/function-item-references.rs stdout ----
diff of stderr:

116   --> $DIR/function-item-references.rs:118:22
117    |
118 LL |     println!("{:p}", &take_generic_array::<u32, 4>);
-    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `take_generic_array` to obtain a function pointer: `take_generic_array::<u32, 4_usize> as fn(_)`
+    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `take_generic_array` to obtain a function pointer: `take_generic_array::<u32, 4> as fn(_)`
121 warning: taking a reference to a function item does not give a function pointer
122   --> $DIR/function-item-references.rs:120:22

128   --> $DIR/function-item-references.rs:122:22
128   --> $DIR/function-item-references.rs:122:22
129    |
130 LL |     println!("{:p}", &multiple_generic_arrays::<u32, f32, 4, 8>);
-    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `multiple_generic_arrays` to obtain a function pointer: `multiple_generic_arrays::<u32, f32, 4_usize, 8_usize> as fn(_, _)`
+    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `multiple_generic_arrays` to obtain a function pointer: `multiple_generic_arrays::<u32, f32, 4, 8> as fn(_, _)`
133 warning: taking a reference to a function item does not give a function pointer
134   --> $DIR/function-item-references.rs:124:22



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/function-item-references/function-item-references.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/function-item-references.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/function-item-references.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/function-item-references" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/function-item-references/auxiliary"
stdout: none
--- stderr -------------------------------
warning: taking a reference to a function item does not give a function pointer
   |
   |
LL |     Pointer::fmt(&zst_ref, f)
   |                  ^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/function-item-references.rs:3:9
   |
LL | #![warn(function_item_references)]
LL | #![warn(function_item_references)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^

warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:81:22
   |
LL |     println!("{:p}", &foo);
   |                      ^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:83:20
   |
LL |     print!("{:p}", &foo);
LL |     print!("{:p}", &foo);
   |                    ^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:85:21
   |
LL |     format!("{:p}", &foo);
LL |     format!("{:p}", &foo);
   |                     ^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:88:22
   |
LL |     println!("{:p}", &foo as *const _);
LL |     println!("{:p}", &foo as *const _);
   |                      ^^^^^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:90:22
   |
   |
LL |     println!("{:p}", zst_ref);
   |                      ^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:92:22
   |
LL |     println!("{:p}", cast_zst_ptr);
LL |     println!("{:p}", cast_zst_ptr);
   |                      ^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:94:22
   |
LL |     println!("{:p}", coerced_zst_ptr);
LL |     println!("{:p}", coerced_zst_ptr);
   |                      ^^^^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:97:22
   |
LL |     println!("{:p}", &fn_item);
LL |     println!("{:p}", &fn_item);
   |                      ^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:99:22
   |
LL |     println!("{:p}", indirect_ref);
LL |     println!("{:p}", indirect_ref);
   |                      ^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:102:22
   |
LL |     println!("{:p}", &nop);
LL |     println!("{:p}", &nop);
   |                      ^^^^ help: cast `nop` to obtain a function pointer: `nop as fn()`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:104:22
   |
LL |     println!("{:p}", &bar);
LL |     println!("{:p}", &bar);
   |                      ^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:106:22
   |
LL |     println!("{:p}", &baz);
LL |     println!("{:p}", &baz);
   |                      ^^^^ help: cast `baz` to obtain a function pointer: `baz as fn(_, _) -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:108:22
   |
   |
LL |     println!("{:p}", &unsafe_fn);
   |                      ^^^^^^^^^^ help: cast `unsafe_fn` to obtain a function pointer: `unsafe_fn as unsafe fn()`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:110:22
   |
   |
LL |     println!("{:p}", &c_fn);
   |                      ^^^^^ help: cast `c_fn` to obtain a function pointer: `c_fn as extern "C" fn()`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:112:22
   |
   |
LL |     println!("{:p}", &unsafe_c_fn);
   |                      ^^^^^^^^^^^^ help: cast `unsafe_c_fn` to obtain a function pointer: `unsafe_c_fn as unsafe extern "C" fn()`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:114:22
   |
LL |     println!("{:p}", &variadic);
LL |     println!("{:p}", &variadic);
   |                      ^^^^^^^^^ help: cast `variadic` to obtain a function pointer: `variadic as unsafe extern "C" fn(_, ...)`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:116:22
   |
   |
LL |     println!("{:p}", &take_generic_ref::<u32>);
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `take_generic_ref` to obtain a function pointer: `take_generic_ref::<u32> as fn(_)`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:118:22
   |
   |
LL |     println!("{:p}", &take_generic_array::<u32, 4>);
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `take_generic_array` to obtain a function pointer: `take_generic_array::<u32, 4> as fn(_)`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:120:22
   |
   |
LL |     println!("{:p}", &multiple_generic::<u32, f32>);
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `multiple_generic` to obtain a function pointer: `multiple_generic::<u32, f32> as fn(_, _)`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:122:22
   |
   |
LL |     println!("{:p}", &multiple_generic_arrays::<u32, f32, 4, 8>);
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `multiple_generic_arrays` to obtain a function pointer: `multiple_generic_arrays::<u32, f32, 4, 8> as fn(_, _)`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:124:22
   |
   |
LL |     println!("{:p}", &std::env::var::<String>);
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `var` to obtain a function pointer: `var::<String> as fn(_) -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:127:32
   |
   |
LL |     println!("{:p} {:p} {:p}", &nop, &foo, &bar);
   |                                ^^^^ help: cast `nop` to obtain a function pointer: `nop as fn()`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:127:38
   |
   |
LL |     println!("{:p} {:p} {:p}", &nop, &foo, &bar);
   |                                      ^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:127:44
   |
   |
LL |     println!("{:p} {:p} {:p}", &nop, &foo, &bar);
   |                                            ^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:142:41
   |
   |
LL |         std::mem::transmute::<_, usize>(&foo);
   |                                         ^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:144:50
   |
   |
LL |         std::mem::transmute::<_, (usize, usize)>((&foo, &bar));
   |                                                  ^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:144:50
   |
   |
LL |         std::mem::transmute::<_, (usize, usize)>((&foo, &bar));
   |                                                  ^^^^^^^^^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:147:41
   |
   |
LL |         std::mem::transmute::<_, usize>(&take_generic_ref::<u32>);
   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `take_generic_ref` to obtain a function pointer: `take_generic_ref::<u32> as fn(_)`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:156:15
   |
   |
LL |     print_ptr(&bar);
   |               ^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:158:24
   |
   |
LL |     bound_by_ptr_trait(&bar);
   |                        ^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:160:30
   |
   |
LL |     bound_by_ptr_trait_tuple((&foo, &bar));
   |                              ^^^^^^^^^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`
warning: taking a reference to a function item does not give a function pointer
  --> /checkout/src/test/ui/lint/function-item-references.rs:160:30
   |
   |
LL |     bound_by_ptr_trait_tuple((&foo, &bar));
   |                              ^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: 33 warnings emitted
------------------------------------------



---- [ui] src/test/ui/methods/method-not-found-generic-arg-elision.rs stdout ----
diff of stderr:

50 LL |     wrapper.other();
51    |             ^^^^^ method not found in `Wrapper<bool>`
52 
- error[E0599]: no method named `method` found for struct `Wrapper2<'_, bool, 3_usize>` in the current scope
+ error[E0599]: no method named `method` found for struct `Wrapper2<'_, bool, 3>` in the current scope
55    |
55    |
56 LL | struct Wrapper2<'a, T, const C: usize> {

57    | -------------------------------------- method `method` not found for this struct
58 ...
59 LL |     wrapper.method();
-    |             ^^^^^^ method not found in `Wrapper2<'_, bool, 3_usize>`
+    |             ^^^^^^ method not found in `Wrapper2<'_, bool, 3>`
62    = note: the method was found for
62    = note: the method was found for
63            - `Wrapper2<'a, i8, C>`

71    | -------------------------------------- method `other` not found for this struct
73 LL |     wrapper.other();
73 LL |     wrapper.other();
-    |             ^^^^^ method not found in `Wrapper2<'_, bool, 3_usize>`
+    |             ^^^^^ method not found in `Wrapper2<'_, bool, 3>`
75 
76 error[E0599]: no method named `not_found` found for struct `Vec<{integer}>` in the current scope


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision/method-not-found-generic-arg-elision.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision/method-not-found-generic-arg-elision.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args methods/method-not-found-generic-arg-elision.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `distance` found for struct `Point<i32>` in the current scope
   |
LL | struct Point<T> {
LL | struct Point<T> {
   | --------------- method `distance` not found for this struct
...
LL |     let d = point_i32.distance();
   |                       ^^^^^^^^ method not found in `Point<i32>`
   = note: the method was found for
   = note: the method was found for
           - `Point<f64>`

error[E0599]: no method named `other` found for struct `Point` in the current scope
   |
LL | struct Point<T> {
LL | struct Point<T> {
   | --------------- method `other` not found for this struct
...
LL |     let d = point_i32.other();
   |                       ^^^^^ method not found in `Point<i32>`
error[E0599]: no method named `extend` found for struct `Map` in the current scope
  --> /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:87:29
   |
   |
LL |     v.iter().map(|x| x * x).extend(std::iter::once(100));
   |                             ^^^^^^ method not found in `Map<std::slice::Iter<'_, i32>, [closure@/checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:87:18: 87:21]>`

error[E0599]: no method named `method` found for struct `Wrapper<bool>` in the current scope
   |
   |
LL | struct Wrapper<T>(T);
   | ----------------- method `method` not found for this struct
...
LL |     wrapper.method();
   |             ^^^^^^ method not found in `Wrapper<bool>`
   = note: the method was found for
   = note: the method was found for
           - `Wrapper<i8>`
           - `Wrapper<i16>`
           - `Wrapper<i32>`
           - `Wrapper<i64>`
           and 2 more types

error[E0599]: no method named `other` found for struct `Wrapper` in the current scope
   |
   |
LL | struct Wrapper<T>(T);
   | ----------------- method `other` not found for this struct
LL |     wrapper.other();
LL |     wrapper.other();
   |             ^^^^^ method not found in `Wrapper<bool>`

error[E0599]: no method named `method` found for struct `Wrapper2<'_, bool, 3>` in the current scope
   |
   |
LL | struct Wrapper2<'a, T, const C: usize> {
   | -------------------------------------- method `method` not found for this struct
...
LL |     wrapper.method();
   |             ^^^^^^ method not found in `Wrapper2<'_, bool, 3>`
   = note: the method was found for
   = note: the method was found for
           - `Wrapper2<'a, i8, C>`
           - `Wrapper2<'a, i16, C>`
           - `Wrapper2<'a, i32, C>`

error[E0599]: no method named `other` found for struct `Wrapper2` in the current scope
   |
   |
LL | struct Wrapper2<'a, T, const C: usize> {
   | -------------------------------------- method `other` not found for this struct
LL |     wrapper.other();
LL |     wrapper.other();
   |             ^^^^^ method not found in `Wrapper2<'_, bool, 3>`

error[E0599]: no method named `not_found` found for struct `Vec<{integer}>` in the current scope
   |
LL |     a.not_found();
LL |     a.not_found();
   |       ^^^^^^^^^ method not found in `Vec<{integer}>`

error[E0599]: the method `method` exists for struct `Struct<f64>`, but its trait bounds were not satisfied
   |
   |
LL | struct Struct<T>{
   | ---------------- method `method` not found for this struct
...
LL |     s.method();
   |       ^^^^^^ method cannot be called on `Struct<f64>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `f64: Eq`
           `f64: Ord`
error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/simd/intrinsic/generic-shuffle.rs stdout ----
diff of stderr:

- error[E0511]: invalid monomorphization of `simd_shuffle` intrinsic: expected return type of length 2, found `Simd<u32, 4_usize>` with length 4
+ error[E0511]: invalid monomorphization of `simd_shuffle` intrinsic: expected return type of length 2, found `Simd<u32, 4>` with length 4
3    |
3    |
4 LL |         let _: Simd<u32, 4> = simd_shuffle(v, v, I);
5    |                               ^^^^^^^^^^^^^^^^^^^^^
6 
6 
- error[E0511]: invalid monomorphization of `simd_shuffle` intrinsic: expected return element type `u32` (element of input `Simd<u32, 4_usize>`), found `Simd<f32, 2_usize>` with element type `f32`
+ error[E0511]: invalid monomorphization of `simd_shuffle` intrinsic: expected return element type `u32` (element of input `Simd<u32, 4>`), found `Simd<f32, 2>` with element type `f32`
9    |
9    |
10 LL |         let _: Simd<f32, 2> = simd_shuffle(v, v, I);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/intrinsic/generic-shuffle/generic-shuffle.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args simd/intrinsic/generic-shuffle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/intrinsic/generic-shuffle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/intrinsic/generic-shuffle" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/intrinsic/generic-shuffle/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0511]: invalid monomorphization of `simd_shuffle` intrinsic: expected return type of length 2, found `Simd<u32, 4>` with length 4
   |
   |
LL |         let _: Simd<u32, 4> = simd_shuffle(v, v, I);


error[E0511]: invalid monomorphization of `simd_shuffle` intrinsic: expected return element type `u32` (element of input `Simd<u32, 4>`), found `Simd<f32, 2>` with element type `f32`
   |
   |
LL |         let _: Simd<f32, 2> = simd_shuffle(v, v, I);


error[E0511]: invalid monomorphization of `simd_shuffle` intrinsic: simd_shuffle index must be an array of `u32`, got `[f32; 2]`
   |
   |
LL |         let _: Simd<u32, 2> = simd_shuffle(v, v, I2);

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0511`.
---
diff of stderr:

2   --> $DIR/libm_no_std_cant_float.rs:14:17
3    |
4 LL |     let _xc = x.ceil();
+    |                 ^^^^ method not found in `Simd<f32, 4>`
6 
6 
7 error[E0599]: no method named `floor` found for struct `Simd` in the current scope

9    |
9    |
10 LL |     let _xf = x.floor();
+    |                 ^^^^^ method not found in `Simd<f32, 4>`
12 
13 error[E0599]: no method named `round` found for struct `Simd` in the current scope
14   --> $DIR/libm_no_std_cant_float.rs:16:17
14   --> $DIR/libm_no_std_cant_float.rs:16:17

15    |
16 LL |     let _xr = x.round();
+    |                 ^^^^^ method not found in `Simd<f32, 4>`
18 
19 error[E0599]: no method named `trunc` found for struct `Simd` in the current scope
20   --> $DIR/libm_no_std_cant_float.rs:17:17
20   --> $DIR/libm_no_std_cant_float.rs:17:17

21    |
22 LL |     let _xt = x.trunc();
+    |                 ^^^^^ method not found in `Simd<f32, 4>`
24 
25 error[E0599]: no method named `mul_add` found for struct `Simd` in the current scope
26   --> $DIR/libm_no_std_cant_float.rs:18:19
26   --> $DIR/libm_no_std_cant_float.rs:18:19

27    |
28 LL |     let _xfma = x.mul_add(x, x);
+    |                   ^^^^^^^ method not found in `Simd<f32, 4>`
30 
30 
31 error[E0599]: no method named `sqrt` found for struct `Simd` in the current scope

33    |
33    |
34 LL |     let _xsqrt = x.sqrt();
+    |                    ^^^^ method not found in `Simd<f32, 4>`
36 
37 error: aborting due to 6 previous errors
38 
---
To only update this specific test, also pass `--test-args simd/libm_no_std_cant_float.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/libm_no_std_cant_float.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/libm_no_std_cant_float" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/libm_no_std_cant_float/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `ceil` found for struct `Simd` in the current scope
   |
   |
LL |     let _xc = x.ceil(); //~ ERROR E0599
   |                 ^^^^ method not found in `Simd<f32, 4>`

error[E0599]: no method named `floor` found for struct `Simd` in the current scope
   |
   |
LL |     let _xf = x.floor(); //~ ERROR E0599
   |                 ^^^^^ method not found in `Simd<f32, 4>`
error[E0599]: no method named `round` found for struct `Simd` in the current scope
  --> /checkout/src/test/ui/simd/libm_no_std_cant_float.rs:16:17
   |
   |
LL |     let _xr = x.round(); //~ ERROR E0599
   |                 ^^^^^ method not found in `Simd<f32, 4>`
error[E0599]: no method named `trunc` found for struct `Simd` in the current scope
  --> /checkout/src/test/ui/simd/libm_no_std_cant_float.rs:17:17
   |
   |
LL |     let _xt = x.trunc(); //~ ERROR E0599
   |                 ^^^^^ method not found in `Simd<f32, 4>`
error[E0599]: no method named `mul_add` found for struct `Simd` in the current scope
  --> /checkout/src/test/ui/simd/libm_no_std_cant_float.rs:18:19
   |
   |
LL |     let _xfma = x.mul_add(x, x); //~ ERROR E0599
   |                   ^^^^^^^ method not found in `Simd<f32, 4>`

error[E0599]: no method named `sqrt` found for struct `Simd` in the current scope
   |
   |
LL |     let _xsqrt = x.sqrt(); //~ ERROR E0599
   |                    ^^^^ method not found in `Simd<f32, 4>`
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/simd/type-generic-monomorphisation-empty.rs stdout ----
diff of stderr:

- error: monomorphising SIMD type `Simd<0_usize>` of zero length
+ error: monomorphising SIMD type `Simd<0>` of zero length
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-generic-monomorphisation-empty/type-generic-monomorphisation-empty.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args simd/type-generic-monomorphisation-empty.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/type-generic-monomorphisation-empty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-generic-monomorphisation-empty" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-generic-monomorphisation-empty/auxiliary"
stdout: none
--- stderr -------------------------------
error: monomorphising SIMD type `Simd<0>` of zero length
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/simd/type-generic-monomorphisation-oversized.rs stdout ----
diff of stderr:

- error: monomorphising SIMD type `Simd<65536_usize>` of length greater than 32768
+ error: monomorphising SIMD type `Simd<65536>` of length greater than 32768
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-generic-monomorphisation-oversized/type-generic-monomorphisation-oversized.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args simd/type-generic-monomorphisation-oversized.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/type-generic-monomorphisation-oversized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-generic-monomorphisation-oversized" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-generic-monomorphisation-oversized/auxiliary"
stdout: none
--- stderr -------------------------------
error: monomorphising SIMD type `Simd<65536>` of length greater than 32768
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs stdout ----
diff of stderr:

25 LL |     7u32
26    |     ^^^^
27    |
- note: used non-generic constant `123_usize` for generic parameter
+ note: used non-generic constant `123` for generic parameter
30    |
30    |
31 LL | type OneConst<const X: usize> = impl Debug;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_nondefining_use/generic_nondefining_use.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_nondefining_use.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_nondefining_use" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_nondefining_use/auxiliary"
stdout: none
--- stderr -------------------------------
error: non-defining opaque type use in defining scope
   |
LL |     5u32
   |     ^^^^
   |
   |
note: used non-generic type `u32` for generic parameter
   |
   |
LL | type OneTy<T> = impl Debug;

error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:22:5
   |
   |
LL | type OneLifetime<'a> = impl Debug;
   |                  -- cannot use static lifetime; use a bound lifetime instead or remove the lifetime parameter from the opaque type
LL |     6u32
   |     ^^^^

error: non-defining opaque type use in defining scope
error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_nondefining_use.rs:27:5
   |
LL |     7u32
   |     ^^^^
   |
note: used non-generic constant `123` for generic parameter
   |
   |
LL | type OneConst<const X: usize> = impl Debug;

error: aborting due to 3 previous errors
------------------------------------------

