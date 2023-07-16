plain
........................................................................................ 1848/13426
........................................................................................ 1936/13426
.................................i...................................................... 2024/13426
........................................................................................ 2112/13426
..F.............................F.............................F......................... 2200/13426
.....................................................................................F.. 2288/13426
......F................................................................................. 2376/13426
........................................................................................ 2552/13426
........................................................................................ 2640/13426
........................................................................................ 2728/13426
........................................................................................ 2816/13426
---
---- [ui] src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs stdout ----
diff of stderr:

24    |
25    = note: expected constant `{ N as u128 }`
26               found constant `{ O as u128 }`
- note: required by a bound in `use_trait_impl::assert_impl`
-   --> $DIR/abstract-const-as-cast-3.rs:14:23
-    |
- LL |     fn assert_impl<T: Trait>() {}
-    |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
33 error: unconstrained generic constant
34   --> $DIR/abstract-const-as-cast-3.rs:20:19

56    |
56    |
57    = note: expected constant `{ N as _ }`
58               found constant `{ O as u128 }`
- note: required by a bound in `use_trait_impl::assert_impl`
-   --> $DIR/abstract-const-as-cast-3.rs:14:23
-    |
- LL |     fn assert_impl<T: Trait>() {}
-    |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
65 error[E0308]: mismatched types
66   --> $DIR/abstract-const-as-cast-3.rs:23:5

70    |
70    |
71    = note: expected constant `12`
72               found constant `13`
- note: required by a bound in `use_trait_impl::assert_impl`
-   --> $DIR/abstract-const-as-cast-3.rs:14:23
-    |
- LL |     fn assert_impl<T: Trait>() {}
-    |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
79 error[E0308]: mismatched types
80   --> $DIR/abstract-const-as-cast-3.rs:25:5

84    |
84    |
85    = note: expected constant `13`
86               found constant `14`
- note: required by a bound in `use_trait_impl::assert_impl`
-   --> $DIR/abstract-const-as-cast-3.rs:14:23
-    |
- LL |     fn assert_impl<T: Trait>() {}
-    |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
93 error: unconstrained generic constant
94   --> $DIR/abstract-const-as-cast-3.rs:35:19

116    |
116    |
117    = note: expected constant `{ N as u128 }`
118               found constant `{ O as u128 }`
- note: required by a bound in `use_trait_impl_2::assert_impl`
-   --> $DIR/abstract-const-as-cast-3.rs:32:23
-    |
- LL |     fn assert_impl<T: Trait>() {}
-    |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
125 error: unconstrained generic constant
126   --> $DIR/abstract-const-as-cast-3.rs:38:19

148    |
148    |
149    = note: expected constant `{ N as _ }`
150               found constant `{ O as u128 }`
- note: required by a bound in `use_trait_impl_2::assert_impl`
-   --> $DIR/abstract-const-as-cast-3.rs:32:23
-    |
- LL |     fn assert_impl<T: Trait>() {}
-    |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
157 error[E0308]: mismatched types
158   --> $DIR/abstract-const-as-cast-3.rs:41:5

162    |
162    |
163    = note: expected constant `12`
164               found constant `13`
- note: required by a bound in `use_trait_impl_2::assert_impl`
-   --> $DIR/abstract-const-as-cast-3.rs:32:23
-    |
- LL |     fn assert_impl<T: Trait>() {}
-    |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
171 error[E0308]: mismatched types
172   --> $DIR/abstract-const-as-cast-3.rs:43:5

176    |
176    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
177    = note: expected constant `13`
178               found constant `14`
- note: required by a bound in `use_trait_impl_2::assert_impl`
-   --> $DIR/abstract-const-as-cast-3.rs:32:23
-    |
- LL |     fn assert_impl<T: Trait>() {}
-    |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
185 error: aborting due to 12 previous errors
186 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3/abstract-const-as-cast-3.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/abstract-const-as-cast-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as u128 }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required for `HasCastInTraitImpl<{ N + 1 }, { N as u128 }>` to implement `Trait`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
note: required by a bound in `use_trait_impl::assert_impl`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:14:23
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:17:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as u128 }`, found `{ O as u128 }`
   |
   = note: expected constant `{ N as u128 }`
              found constant `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:20:19
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as _ }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required for `HasCastInTraitImpl<{ N + 1 }, { N as _ }>` to implement `Trait`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
note: required by a bound in `use_trait_impl::assert_impl`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:14:23
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:20:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as _ }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as _ }`, found `{ O as u128 }`
   |
   = note: expected constant `{ N as _ }`
              found constant `{ O as u128 }`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:23:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<13, { 12 as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `12`, found `13`
   = note: expected constant `12`
              found constant `13`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:25:5
   |
LL |     assert_impl::<HasCastInTraitImpl<14, 13>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `13`, found `14`
   = note: expected constant `13`
              found constant `14`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:35:19
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as u128 }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required for `HasCastInTraitImpl<{ N + 1 }, { N as u128 }>` to implement `Trait`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `use_trait_impl_2::assert_impl`
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:35:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as u128 }`, found `{ O as u128 }`
   |
   = note: expected constant `{ N as u128 }`
              found constant `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:38:19
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as _ }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required for `HasCastInTraitImpl<{ N + 1 }, { N as _ }>` to implement `Trait`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `use_trait_impl_2::assert_impl`
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:38:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as _ }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as _ }`, found `{ O as u128 }`
   |
   = note: expected constant `{ N as _ }`
              found constant `{ O as u128 }`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:41:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<13, { 12 as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `12`, found `13`
   = note: expected constant `12`
              found constant `13`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:43:5
   |
LL |     assert_impl::<HasCastInTraitImpl<14, 13>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `13`, found `14`
   = note: expected constant `13`
              found constant `14`

error: aborting due to 12 previous errors
---
24               found constant `true`
- note: required by a bound in `Arr`
-   --> $DIR/issue-72819-generic-in-const-eval.rs:8:39
-    |
- LL | struct Arr<const N: usize>
-    |        --- required by a bound in this
- LL | where Assert::<{N < usize::MAX / 2}>: IsTrue,
-    |                                       ^^^^^^ required by this bound in `Arr`
33 error: aborting due to 2 previous errors
34 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.full/issue-72819-generic-in-const-eval.full.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.full/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.rs:20:12
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |            ^^^^^^^^^^^^^^^^^ expected `false`, found `true`
   = note: expected constant `false`
              found constant `true`
note: required by a bound in `Arr`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.rs:8:39
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.rs:8:39
   |
LL | struct Arr<const N: usize>
   |        --- required by a bound in this
LL | where Assert::<{N < usize::MAX / 2}>: IsTrue,
   |                                       ^^^^^^ required by this bound in `Arr`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.rs:20:32
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |                                ^^^ expected `false`, found `true`
   = note: expected constant `false`
              found constant `true`

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/obligation-cause.rs stdout ----

6    |
7    = note: expected constant `false`
8               found constant `true`
8               found constant `true`
- note: required by a bound in `g`
-   --> $DIR/obligation-cause.rs:13:44
-    |
- LL | fn g<T>()
- ...
- ...
- LL |     Is<{ std::mem::size_of::<T>() == 0 }>: True,
-    |                                            ^^^^ required by this bound in `g`
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/obligation-cause/obligation-cause.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/obligation-cause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/obligation-cause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/obligation-cause" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/obligation-cause/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/obligation-cause.rs:20:5
   |
   |
LL |     g::<usize>();
   |     ^^^^^^^^^^ expected `false`, found `true`
   = note: expected constant `false`
              found constant `true`

error: aborting due to previous error
---
25               found constant `true`
- note: required by a bound in `Arr`
-   --> $DIR/issue-73260.rs:6:37
-    |
- LL | struct Arr<const N: usize>
- LL | where
- LL | where
- LL |     Assert::<{N < usize::MAX / 2}>: IsTrue,
-    |                                     ^^^^^^ required by this bound in `Arr`
35 error: aborting due to 2 previous errors
36 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-73260/issue-73260.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-73260.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-73260.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-73260" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-73260/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/issues/issue-73260.rs:16:12
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |            ^^^^^^^^^^^^^^^^^ expected `false`, found `true`
   = note: expected constant `false`
              found constant `true`
note: required by a bound in `Arr`
  --> /checkout/src/test/ui/const-generics/issues/issue-73260.rs:6:37
  --> /checkout/src/test/ui/const-generics/issues/issue-73260.rs:6:37
   |
LL | struct Arr<const N: usize>
LL | where
LL | where
LL |     Assert::<{N < usize::MAX / 2}>: IsTrue,
   |                                     ^^^^^^ required by this bound in `Arr`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-73260.rs:16:32
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |                                ^^^ expected `false`, found `true`
   = note: expected constant `false`
              found constant `true`

error: aborting due to 2 previous errors
---
8               found constant `false`
- note: required by a bound in `requires_distinct`
-   --> $DIR/issue-79674.rs:23:37
-    |
- LL | fn requires_distinct<A, B>(_a: A, _b: B) where
-    |    ----------------- required by a bound in this
- LL |     A: MiniTypeId, B: MiniTypeId,
- LL |     Lift<{is_same_type::<A, B>()}>: IsFalse {}
-    |                                     ^^^^^^^ required by this bound in `requires_distinct`
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-79674/issue-79674.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-79674.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-79674.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-79674" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-79674/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/issues/issue-79674.rs:26:5
   |
   |
LL |     requires_distinct("str", 12);
   |     ^^^^^^^^^^^^^^^^^ expected `true`, found `false`
   = note: expected constant `true`
              found constant `false`

error: aborting due to previous error
