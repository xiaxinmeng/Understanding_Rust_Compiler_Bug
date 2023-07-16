plain
diff of stderr:

10    |               ~~~
11 
12 error[E0277]: the trait bound `u32: Foo` is not satisfied
-   --> $DIR/associated-types-path-2.rs:29:14
14    |
15 LL |     f1(2u32, 4u32);
-    |     --       ^^^^ the trait `Foo` is not implemented for `u32`
-    |     |
-    |     |
-    |     required by a bound introduced by this call
+    |     ^^ the trait `Foo` is not implemented for `u32`
19    |
20 note: required by a bound in `f1`
21   --> $DIR/associated-types-path-2.rs:13:14

24    |              ^^^ required by this bound in `f1`
25 
26 error[E0277]: the trait bound `u32: Foo` is not satisfied
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/associated-types-path-2.rs:34:14
28    |
28    |
+ LL |     f1(2u32, 4u32);
+    |     ^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `u32`
+ 
+ error[E0277]: the trait bound `u32: Foo` is not satisfied
+    |
+    |
+ LL |     f1(2u32, 4u32);
+    |              ^^^^ the trait `Foo` is not implemented for `u32`
+ 
+ error[E0277]: the trait bound `u32: Foo` is not satisfied
+    |
+    |
29 LL |     f1(2u32, 4i32);
-    |     --       ^^^^ the trait `Foo` is not implemented for `u32`
+    |     -- ^^^^ the trait `Foo` is not implemented for `u32`
32    |     required by a bound introduced by this call
33    |


37 LL | pub fn f1<T: Foo>(a: T, x: T::A) {}
38    |              ^^^ required by this bound in `f1`
39 
+ error[E0277]: the trait bound `u32: Foo` is not satisfied
+    |
+    |
+ LL |     f1(2u32, 4i32);
+    |              ^^^^ the trait `Foo` is not implemented for `u32`
40 error[E0308]: mismatched types
41   --> $DIR/associated-types-path-2.rs:39:18
42    |


50 LL |     let _: i32 = f2(2i32).try_into().unwrap();
52 
- error: aborting due to 4 previous errors
+ error: aborting due to 7 previous errors
54 
54 
55 Some errors have detailed explanations: E0277, E0308.
56 For more information about an error, try `rustc --explain E0277`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-2/associated-types-path-2.stderr
To only update this specific test, also pass `--test-args associated-types/associated-types-path-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-path-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/associated-types-path-2.rs:19:14
   |
LL |     f1(2i32, 4i32);
   |              ^^^^ expected `u32`, found `i32`
   |
help: change the type of the numeric literal from `i32` to `u32`
   |
LL |     f1(2i32, 4u32);


error[E0277]: the trait bound `u32: Foo` is not satisfied
   |
LL |     f1(2u32, 4u32);
   |     ^^ the trait `Foo` is not implemented for `u32`
   |
   |
note: required by a bound in `f1`
  --> /checkout/src/test/ui/associated-types/associated-types-path-2.rs:13:14
   |
LL | pub fn f1<T: Foo>(a: T, x: T::A) {}
   |              ^^^ required by this bound in `f1`

error[E0277]: the trait bound `u32: Foo` is not satisfied
   |
LL |     f1(2u32, 4u32);
   |     ^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `u32`


error[E0277]: the trait bound `u32: Foo` is not satisfied
   |
LL |     f1(2u32, 4u32);
   |              ^^^^ the trait `Foo` is not implemented for `u32`


error[E0277]: the trait bound `u32: Foo` is not satisfied
   |
   |
LL |     f1(2u32, 4i32);
   |     -- ^^^^ the trait `Foo` is not implemented for `u32`
   |     required by a bound introduced by this call
   |
note: required by a bound in `f1`
  --> /checkout/src/test/ui/associated-types/associated-types-path-2.rs:13:14
  --> /checkout/src/test/ui/associated-types/associated-types-path-2.rs:13:14
   |
LL | pub fn f1<T: Foo>(a: T, x: T::A) {}
   |              ^^^ required by this bound in `f1`

error[E0277]: the trait bound `u32: Foo` is not satisfied
   |
   |
LL |     f1(2u32, 4i32);
   |              ^^^^ the trait `Foo` is not implemented for `u32`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/associated-types-path-2.rs:39:18
   |
   |
LL |     let _: i32 = f2(2i32);
   |            ---   ^^^^^^^^ expected `i32`, found `u32`
   |            expected due to this
   |
   |
help: you can convert a `u32` to an `i32` and panic if the converted value doesn't fit
   |
LL |     let _: i32 = f2(2i32).try_into().unwrap();

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0277, E0308.
---

---- [ui] ui/const-generics/generic_const_exprs/issue-85848.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `(): _Contains<&C>` is not satisfied
-   --> $DIR/issue-85848.rs:24:5
3    |
3    |
4 LL |     writes_to_specific_path(&cap);
-    |     ^^^^^^^^^^^^^^^^^^^^^^^ the trait `_Contains<&C>` is not implemented for `()`
+    |     ----------------------- ^^^^ the trait `_Contains<&C>` is not implemented for `()`
+    |     required by a bound introduced by this call
6    |
6    |
7 note: required because of the requirements on the impl of `Contains<(), true>` for `&C`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-85848/issue-85848.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-85848/issue-85848.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-85848.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-85848" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-85848/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `(): _Contains<&C>` is not satisfied
   |
   |
LL |     writes_to_specific_path(&cap);
   |     ----------------------- ^^^^ the trait `_Contains<&C>` is not implemented for `()`
   |     required by a bound introduced by this call
   |
   |
note: required because of the requirements on the impl of `Contains<(), true>` for `&C`
   |
   |
LL | impl<T, U> Contains<T, { contains::<T, U>() }> for U where T: _Contains<U> {}
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^
note: required because of the requirements on the impl of `Delegates<()>` for `&C`
   |
   |
LL | impl<T, U> Delegates<U> for T where T: Contains<U, true> {}
   |            ^^^^^^^^^^^^     ^
note: required by a bound in `writes_to_specific_path`
   |
   |
LL | fn writes_to_specific_path<C: Delegates<()>>(cap: &C) {}
   |                               ^^^^^^^^^^^^^ required by this bound in `writes_to_specific_path`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs:24:5
   |
   |
LL |     writes_to_specific_path(&cap);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { contains::<T, U>() }]:`
note: required because of the requirements on the impl of `Contains<(), true>` for `&C`
   |
   |
LL | impl<T, U> Contains<T, { contains::<T, U>() }> for U where T: _Contains<U> {}
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^
note: required because of the requirements on the impl of `Delegates<()>` for `&C`
   |
   |
LL | impl<T, U> Delegates<U> for T where T: Contains<U, true> {}
   |            ^^^^^^^^^^^^     ^
note: required by a bound in `writes_to_specific_path`
   |
   |
LL | fn writes_to_specific_path<C: Delegates<()>>(cap: &C) {}
   |                               ^^^^^^^^^^^^^ required by this bound in `writes_to_specific_path`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/intrinsics/const-eval-select-bad.rs stdout ----
diff of stderr:

1 error[E0277]: expected a `FnOnce<()>` closure, found `[closure@$DIR/const-eval-select-bad.rs:6:27: 6:32]`
-   --> $DIR/const-eval-select-bad.rs:6:34
3    |
3    |
4 LL |     const_eval_select((), || {}, || {});
-    |     -----------------            ^^^^^ expected an `FnOnce<()>` closure, found `[closure@$DIR/const-eval-select-bad.rs:6:27: 6:32]`
+    |     -----------------     ^^^^^ expected an `FnOnce<()>` closure, found `[closure@$DIR/const-eval-select-bad.rs:6:27: 6:32]`
7    |     required by a bound introduced by this call
8    |

15    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
15    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
16 
17 error[E0277]: expected a `FnOnce<()>` closure, found `{integer}`
-   --> $DIR/const-eval-select-bad.rs:8:31
19    |
19    |
20 LL |     const_eval_select((), 42, 0xDEADBEEF);
-    |     -----------------         ^^^^^^^^^^ expected an `FnOnce<()>` closure, found `{integer}`
+    |     -----------------     ^^ expected an `FnOnce<()>` closure, found `{integer}`
23    |     required by a bound introduced by this call
24    |


30 LL |     F: ~const FnOnce<ARG, Output = RET>,
31    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
32 
+ error[E0277]: expected a `FnOnce<()>` closure, found `{integer}`
+    |
+    |
+ LL |     const_eval_select((), 42, 0xDEADBEEF);
+    |     -----------------         ^^^^^^^^^^ expected an `FnOnce<()>` closure, found `{integer}`
+    |     required by a bound introduced by this call
+    |
+    |
+    = help: the trait `FnOnce<()>` is not implemented for `{integer}`
+    = note: wrap the `{integer}` in a closure with no arguments: `|| { /* code */ }`
+ note: required by a bound in `const_eval_select`
+   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
+    |
+ LL |     G: FnOnce<ARG, Output = RET> + ~const Drop,
+    |        ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
+ 
33 error[E0271]: type mismatch resolving `<fn(i32) -> bool {bar} as FnOnce<(i32,)>>::Output == i32`
35    |

43    |                    ^^^^^^^^^^^^ required by this bound in `const_eval_select`
44 
44 
45 error[E0631]: type mismatch in function arguments
-   --> $DIR/const-eval-select-bad.rs:32:37
+   --> $DIR/const-eval-select-bad.rs:32:32
47    |
48 LL | const fn foo(n: i32) -> i32 {
49    | --------------------------- found signature of `fn(i32) -> _`
50 ...
50 ...
51 LL |     const_eval_select((true,), foo, baz);
-    |     -----------------               ^^^ expected signature of `fn(bool) -> _`
+    |     -----------------          ^^^ expected signature of `fn(bool) -> _`
54    |     required by a bound introduced by this call
55    |


59 LL |     F: ~const FnOnce<ARG, Output = RET>,
60    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
- error: aborting due to 4 previous errors
+ error: aborting due to 5 previous errors
63 
64 Some errors have detailed explanations: E0271, E0277, E0631.
---
To only update this specific test, also pass `--test-args intrinsics/const-eval-select-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/const-eval-select-bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/const-eval-select-bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: expected a `FnOnce<()>` closure, found `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:6:27: 6:32]`
   |
   |
LL |     const_eval_select((), || {}, || {});
   |     -----------------     ^^^^^ expected an `FnOnce<()>` closure, found `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:6:27: 6:32]`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `FnOnce<()>` is not implemented for `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:6:27: 6:32]`
   = note: wrap the `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:6:27: 6:32]` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `const_eval_select`
   |
   |
LL |     F: ~const FnOnce<ARG, Output = RET>,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`

error[E0277]: expected a `FnOnce<()>` closure, found `{integer}`
   |
   |
LL |     const_eval_select((), 42, 0xDEADBEEF);
   |     -----------------     ^^ expected an `FnOnce<()>` closure, found `{integer}`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `FnOnce<()>` is not implemented for `{integer}`
   = note: wrap the `{integer}` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `const_eval_select`
   |
   |
LL |     F: ~const FnOnce<ARG, Output = RET>,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`

error[E0277]: expected a `FnOnce<()>` closure, found `{integer}`
   |
   |
LL |     const_eval_select((), 42, 0xDEADBEEF);
   |     -----------------         ^^^^^^^^^^ expected an `FnOnce<()>` closure, found `{integer}`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `FnOnce<()>` is not implemented for `{integer}`
   = note: wrap the `{integer}` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `const_eval_select`
   |
   |
LL |     G: FnOnce<ARG, Output = RET> + ~const Drop,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`

error[E0271]: type mismatch resolving `<fn(i32) -> bool {bar} as FnOnce<(i32,)>>::Output == i32`
   |
   |
LL |     const_eval_select((1,), foo, bar);
   |     ^^^^^^^^^^^^^^^^^ expected `i32`, found `bool`
note: required by a bound in `const_eval_select`
  --> /checkout/library/core/src/intrinsics.rs:2271:20
   |
   |
LL |     G: FnOnce<ARG, Output = RET> + ~const Drop,
   |                    ^^^^^^^^^^^^ required by this bound in `const_eval_select`
error[E0631]: type mismatch in function arguments
  --> /checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:32:32
   |
   |
LL | const fn foo(n: i32) -> i32 {
   | --------------------------- found signature of `fn(i32) -> _`
...
LL |     const_eval_select((true,), foo, baz);
   |     -----------------          ^^^ expected signature of `fn(bool) -> _`
   |     required by a bound introduced by this call
   |
note: required by a bound in `const_eval_select`
  --> /checkout/library/core/src/intrinsics.rs:2270:8
  --> /checkout/library/core/src/intrinsics.rs:2270:8
   |
LL |     F: ~const FnOnce<ARG, Output = RET>,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0271, E0277, E0631.
For more information about an error, try `rustc --explain E0271`.
For more information about an error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs stdout ----
diff of stderr:

30    |     ^^^^^^^^^^^^^^^^ `fn() -> i32 {a}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
31    |
32    = help: the trait `Debug` is not implemented for `fn() -> i32 {a}`
-    = help: use parentheses to call the function: `a()`
34    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: use parentheses to call the function
+    |
+ LL |                     $crate::panicking::assert_failed(kind, &*left_val(), &*right_val, $crate::option::Option::None);
35 
36 error: aborting due to 3 previous errors
37 

---
To only update this specific test, also pass `--test-args issues/issue-70724-add_type_neq_err_label-unwrap.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: binary operation `==` cannot be applied to type `fn() -> i32 {a}`
   |
   |
LL |     assert_eq!(a, 0);
   |     |
   |     fn() -> i32 {a}
   |     {integer}
   |     {integer}
   |     help: you might have forgotten to call this function: `*left_val()`
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs:6:5
   |
   |
LL |     assert_eq!(a, 0);
   |     ^^^^^^^^^^^^^^^^ expected fn item, found integer
   |
   = note: expected fn item `fn() -> i32 {a}`
                 found type `i32`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `fn() -> i32 {a}` doesn't implement `Debug`
   |
   |
LL | fn a() -> i32 {
...
...
LL |     assert_eq!(a, 0);
   |     ^^^^^^^^^^^^^^^^ `fn() -> i32 {a}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `fn() -> i32 {a}`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
   |
   |
LL |                     $crate::panicking::assert_failed(kind, &*left_val(), &*right_val, $crate::option::Option::None);

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308, E0369.
---

---- [ui] ui/phantom-auto-trait.rs stdout ----
diff of stderr:

1 error[E0277]: `T` cannot be shared between threads safely
-   --> $DIR/phantom-auto-trait.rs:21:12
3    |
3    |
4 LL |     is_zen(x)
-    |     ------ ^ `T` cannot be shared between threads safely
-    |     required by a bound introduced by this call
-    |     required by a bound introduced by this call
+    |     ^^^^^^ `T` cannot be shared between threads safely
8    |
9 note: required because of the requirements on the impl of `Zen` for `&T`

28    |              +++++++++++++++++++
29 
29 
30 error[E0277]: `T` cannot be shared between threads safely
-   --> $DIR/phantom-auto-trait.rs:26:12
32    |
32    |
33 LL |     is_zen(x)
-    |     ------ ^ `T` cannot be shared between threads safely
-    |     required by a bound introduced by this call
-    |     required by a bound introduced by this call
+    |     ^^^^^^ `T` cannot be shared between threads safely
37    |
38 note: required because of the requirements on the impl of `Zen` for `&T`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/phantom-auto-trait/phantom-auto-trait.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/phantom-auto-trait/phantom-auto-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args phantom-auto-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/phantom-auto-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/phantom-auto-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/phantom-auto-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `T` cannot be shared between threads safely
   |
   |
LL |     is_zen(x)
   |     ^^^^^^ `T` cannot be shared between threads safely
   |
note: required because of the requirements on the impl of `Zen` for `&T`
   |
   |
LL | unsafe impl<'a, T: 'a> Zen for &'a T where T: Sync {}
   |                        ^^^     ^^^^^
   = note: required because it appears within the type `PhantomData<&T>`
note: required because it appears within the type `Guard<'_, T>`
   |
   |
LL | struct Guard<'a, T: 'a> {
   |        ^^^^^
note: required by a bound in `is_zen`
   |
   |
LL | fn is_zen<T: Zen>(_: T) {}
   |              ^^^ required by this bound in `is_zen`
help: consider restricting type parameter `T`
   |
LL | fn not_sync<T: std::marker::Sync>(x: Guard<T>) {


error[E0277]: `T` cannot be shared between threads safely
   |
   |
LL |     is_zen(x)
   |     ^^^^^^ `T` cannot be shared between threads safely
   |
note: required because of the requirements on the impl of `Zen` for `&T`
   |
   |
LL | unsafe impl<'a, T: 'a> Zen for &'a T where T: Sync {}
   |                        ^^^     ^^^^^
   = note: required because it appears within the type `PhantomData<&T>`
note: required because it appears within the type `Guard<'_, T>`
   |
   |
LL | struct Guard<'a, T: 'a> {
   |        ^^^^^
note: required because it appears within the type `Nested<Guard<'_, T>>`
   |
   |
LL | struct Nested<T>(T);
   |        ^^^^^^
note: required by a bound in `is_zen`
   |
   |
LL | fn is_zen<T: Zen>(_: T) {}
   |              ^^^ required by this bound in `is_zen`
help: consider restricting type parameter `T`
   |
LL | fn nested_not_sync<T: std::marker::Sync>(x: Nested<Guard<T>>) {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/rfc-2632-const-trait-impl/const-drop-fail.rs#precise stdout ----
diff of stderr:

51 error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
53    |
53    |
- LL |         const _: () = check($exp);
-    |                       ----- required by a bound introduced by this call
- ...
57 LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
58    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.precise/const-drop-fail.precise.stderr
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-drop-fail.rs`


error in revision `precise`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "precise" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.precise" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.precise/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `~const` is not allowed here
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);
   |
   |
   = note: only allowed on bounds on traits' associated types and functions, const fns, const impls and its associated functions

error[E0277]: the trait bound `NonTrivialDrop: Drop` is not satisfied
   |
   |
LL |         const _: () = check($exp);
   |                       ----- required by a bound introduced by this call
...
LL |     NonTrivialDrop,
   |     ^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `NonTrivialDrop`
note: required by a bound in `check`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs:36:19
   |
   |
LL | const fn check<T: ~const Drop>(_: T) {}
   |                   ^^^^^^^^^^^ required by this bound in `check`

error[E0277]: the trait bound `ConstImplWithDropGlue: Drop` is not satisfied
   |
   |
LL |         const _: () = check($exp);
   |                       ----- required by a bound introduced by this call
...
LL |     ConstImplWithDropGlue(NonTrivialDrop),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `ConstImplWithDropGlue`
note: required by a bound in `check`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs:36:19
   |
   |
LL | const fn check<T: ~const Drop>(_: T) {}
   |                   ^^^^^^^^^^^ required by this bound in `check`

error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
   |
   |
LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`
   |
note: required by `ConstDropImplWithBounds`
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);


error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
   |
   |
LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`
   |
note: required by a bound in `ConstDropImplWithBounds`
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);
   |                                   ^^^^^^^^ required by this bound in `ConstDropImplWithBounds`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/rfc-2632-const-trait-impl/const-drop-fail.rs#stock stdout ----
diff of stderr:

51 error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
53    |
53    |
- LL |         const _: () = check($exp);
-    |                       ----- required by a bound introduced by this call
- ...
57 LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
58    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.stock/const-drop-fail.stock.stderr
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-drop-fail.rs`


error in revision `stock`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.stock" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.stock/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `~const` is not allowed here
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);
   |
   |
   = note: only allowed on bounds on traits' associated types and functions, const fns, const impls and its associated functions

error[E0277]: the trait bound `NonTrivialDrop: Drop` is not satisfied
   |
   |
LL |         const _: () = check($exp);
   |                       ----- required by a bound introduced by this call
...
LL |     NonTrivialDrop,
   |     ^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `NonTrivialDrop`
note: required by a bound in `check`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs:36:19
   |
   |
LL | const fn check<T: ~const Drop>(_: T) {}
   |                   ^^^^^^^^^^^ required by this bound in `check`

error[E0277]: the trait bound `ConstImplWithDropGlue: Drop` is not satisfied
   |
   |
LL |         const _: () = check($exp);
   |                       ----- required by a bound introduced by this call
...
LL |     ConstImplWithDropGlue(NonTrivialDrop),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `ConstImplWithDropGlue`
note: required by a bound in `check`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs:36:19
   |
   |
LL | const fn check<T: ~const Drop>(_: T) {}
   |                   ^^^^^^^^^^^ required by this bound in `check`

error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
   |
   |
LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`
   |
note: required by `ConstDropImplWithBounds`
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);


error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
   |
   |
LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`
   |
note: required by a bound in `ConstDropImplWithBounds`
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);
   |                                   ^^^^^^^^ required by this bound in `ConstDropImplWithBounds`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/suggestions/into-str.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `&str: From<String>` is not satisfied
-   --> $DIR/into-str.rs:4:5
3    |
4 LL |     foo(String::new());
4 LL |     foo(String::new());
-    |     ^^^ the trait `From<String>` is not implemented for `&str`
+    |     --- ^^^^^^^^^^^^^ the trait `From<String>` is not implemented for `&str`
+    |     required by a bound introduced by this call
6    |
6    |
7    = note: to coerce a `String` into a `&str`, use `&*` as a prefix
8    = note: required because of the requirements on the impl of `Into<&str>` for `String`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str/into-str.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/into-str.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/into-str.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `&str: From<String>` is not satisfied
   |
LL |     foo(String::new());
LL |     foo(String::new());
   |     --- ^^^^^^^^^^^^^ the trait `From<String>` is not implemented for `&str`
   |     required by a bound introduced by this call
   |
   |
   = note: to coerce a `String` into a `&str`, use `&*` as a prefix
   = note: required because of the requirements on the impl of `Into<&str>` for `String`
note: required by a bound in `foo`
   |
   |
LL | fn foo<'a, T>(_t: T) where T: Into<&'a str> {}
   |                               ^^^^^^^^^^^^^ required by this bound in `foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/negative-impls/negated-auto-traits-error.rs stdout ----
diff of stderr:

58    |               ^^^^ required by this bound in `is_send`
59 
60 error[E0277]: `dummy2::TestType` cannot be sent between threads safely
-   --> $DIR/negated-auto-traits-error.rs:48:13
62    |
62    |
63 LL |     is_send(Box::new(TestType));
-    |     ------- ^^^^^^^^^^^^^^^^^^ expected an implementor of trait `Send`
-    |     required by a bound introduced by this call
-    |     required by a bound introduced by this call
+    |     ^^^^^^^ `dummy2::TestType` cannot be sent between threads safely
67    |
-    = note: the trait bound `dummy2::TestType: Send` is not satisfied
+    = help: the trait `Send` is not implemented for `dummy2::TestType`
69    = note: required because of the requirements on the impl of `Send` for `Unique<dummy2::TestType>`
70    = note: required because it appears within the type `Box<dummy2::TestType>`
71 note: required by a bound in `is_send`
73    |
73    |
74 LL | fn is_send<T: Send>(_: T) {}
75    |               ^^^^ required by this bound in `is_send`
- help: consider borrowing here
-    |
- LL |     is_send(&Box::new(TestType));
80 
80 
81 error[E0277]: `dummy3::TestType` cannot be sent between threads safely


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/negative-impls/negated-auto-traits-error/negated-auto-traits-error.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/negative-impls/negated-auto-traits-error/negated-auto-traits-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/negative-impls/negated-auto-traits-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/negative-impls/negated-auto-traits-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/negative-impls/negated-auto-traits-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/negative-impls/negated-auto-traits-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `dummy::TestType` cannot be sent between threads safely
   |
LL |     Outer(TestType);
LL |     Outer(TestType);
   |     ----- ^^^^^^^^ `dummy::TestType` cannot be sent between threads safely
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Send` is not implemented for `dummy::TestType`
note: required by `Outer`
   |
   |
LL | struct Outer<T: Send>(T);


error[E0277]: `dummy::TestType` cannot be sent between threads safely
   |
LL |     Outer(TestType);
LL |     Outer(TestType);
   |     ^^^^^^^^^^^^^^^ `dummy::TestType` cannot be sent between threads safely
   |
   = help: the trait `Send` is not implemented for `dummy::TestType`
note: required by a bound in `Outer`
   |
   |
LL | struct Outer<T: Send>(T);
   |                 ^^^^ required by this bound in `Outer`

error[E0277]: `dummy1b::TestType` cannot be sent between threads safely
   |
LL |     is_send(TestType);
LL |     is_send(TestType);
   |     ------- ^^^^^^^^ `dummy1b::TestType` cannot be sent between threads safely
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Send` is not implemented for `dummy1b::TestType`
note: required by a bound in `is_send`
   |
   |
LL | fn is_send<T: Send>(_: T) {}
   |               ^^^^ required by this bound in `is_send`

error[E0277]: `dummy1c::TestType` cannot be sent between threads safely
   |
   |
LL |     is_send((8, TestType));
   |     ------- ^^^^^^^^^^^^^ `dummy1c::TestType` cannot be sent between threads safely
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Send` is not implemented for `dummy1c::TestType`
   = note: required because it appears within the type `({integer}, dummy1c::TestType)`
note: required by a bound in `is_send`
   |
   |
LL | fn is_send<T: Send>(_: T) {}
   |               ^^^^ required by this bound in `is_send`

error[E0277]: `dummy2::TestType` cannot be sent between threads safely
   |
   |
LL |     is_send(Box::new(TestType));
   |     ^^^^^^^ `dummy2::TestType` cannot be sent between threads safely
   |
   = help: the trait `Send` is not implemented for `dummy2::TestType`
   = note: required because of the requirements on the impl of `Send` for `Unique<dummy2::TestType>`
   = note: required because it appears within the type `Box<dummy2::TestType>`
note: required by a bound in `is_send`
   |
   |
LL | fn is_send<T: Send>(_: T) {}
   |               ^^^^ required by this bound in `is_send`

error[E0277]: `dummy3::TestType` cannot be sent between threads safely
   |
   |
LL |     is_send(Box::new(Outer2(TestType)));
   |     ------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `dummy3::TestType` cannot be sent between threads safely
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Send` is not implemented for `dummy3::TestType`
note: required because it appears within the type `Outer2<dummy3::TestType>`
   |
   |
LL | struct Outer2<T>(T);
   |        ^^^^^^
   = note: required because of the requirements on the impl of `Send` for `Unique<Outer2<dummy3::TestType>>`
   = note: required because it appears within the type `Box<Outer2<dummy3::TestType>>`
note: required by a bound in `is_send`
   |
---
test result: FAILED. 12197 passed; 9 failed; 117 ignored; 0 measured; 0 filtered out; finished in 126.18s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:26
