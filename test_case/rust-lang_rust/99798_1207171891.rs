plain
...................................................i......ii............................ 1760/13327
........................................................................................ 1848/13327
........................................................................................ 1936/13327
......................i................................................................. 2024/13327
......................F...F..............................................F..F.F....F...F 2112/13327
.FFF.FF...FF...FF.F.....FFF..FF.....FF....FFFF.FFFFFFF..F..F...................F........ 2200/13327
.................FF.F.......................................F............F.F....FF.F..F. 2288/13327
.F.F...F..........................F.......F.........F............F.....F.....F.......... 2376/13327
.........................................F.............................................. 2464/13327
........................................................................................ 2640/13327
........................................................................................ 2728/13327
........................................................................................ 2816/13327
........................................................................................ 2904/13327
---
---- [ui] src/test/ui/associated-types/issue-88856.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-88856.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-88856" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-88856/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     sample(t,|x|x.0);
   |     |
   |     required by a bound introduced by this call
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N/2]:`
note: required because of the requirements on the impl of `Trait` for `TraitImpl<10>`
   |
   |
LL | impl<const N:usize> Trait for TraitImpl<N>

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/async-await/interior-with-const-generic-expr.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/interior-with-const-generic-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/interior-with-const-generic-expr/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/interior-with-const-generic-expr/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let ab = concat(a,b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `concat`
   |
   |
LL | fn concat<const A: usize, const B: usize>(a: [f32; A], b: [f32; B]) -> [f32; A + B] {
   |                                                                              ^^^^^ required by this bound in `concat`
error: unconstrained generic constant
  --> /checkout/src/test/ui/async-await/interior-with-const-generic-expr.rs:23:14
   |
   |
LL |     let ab = concat(a,b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `concat`
   |
   |
LL | fn concat<const A: usize, const B: usize>(a: [f32; A], b: [f32; B]) -> [f32; A + B] {
   |                                                                              ^^^^^ required by this bound in `concat`
error: unconstrained generic constant
  --> /checkout/src/test/ui/async-await/interior-with-const-generic-expr.rs:24:14
   |
   |
LL |     let ba = reverse(ab).await;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `concat`
   |
   |
LL | fn concat<const A: usize, const B: usize>(a: [f32; A], b: [f32; B]) -> [f32; A + B] {
   |                                                                              ^^^^^ required by this bound in `concat`
error: unconstrained generic constant
  --> /checkout/src/test/ui/async-await/interior-with-const-generic-expr.rs:24:14
   |
   |
LL |     let ba = reverse(ab).await;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `concat`
   |
   |
LL | fn concat<const A: usize, const B: usize>(a: [f32; A], b: [f32; B]) -> [f32; A + B] {
   |                                                                              ^^^^^ required by this bound in `concat`
error: unconstrained generic constant
  --> /checkout/src/test/ui/async-await/interior-with-const-generic-expr.rs:24:25
   |
   |
LL |     let ba = reverse(ab).await;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `concat`
   |
   |
LL | fn concat<const A: usize, const B: usize>(a: [f32; A], b: [f32; B]) -> [f32; A + B] {
   |                                                                              ^^^^^ required by this bound in `concat`
error: unconstrained generic constant
  --> /checkout/src/test/ui/async-await/interior-with-const-generic-expr.rs:25:22
   |
LL |     println!("{:?}", ba);
LL |     println!("{:?}", ba);
   |                      ^^
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `concat`
   |
   |
LL | fn concat<const A: usize, const B: usize>(a: [f32; A], b: [f32; B]) -> [f32; A + B] {
   |                                                                              ^^^^^ required by this bound in `concat`
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 6 previous errors
------------------------------------------


---
-   --> $DIR/generic-expr-default-concrete.rs:10:5
+ error: unconstrained generic constant
+   --> $DIR/generic-expr-default-concrete.rs:5:25
3    |
- LL |     Foo::<10, 12>
-    |     ^^^^^^^^^^^^^ expected `11`, found `12`
+ LL | fn no_constraining() -> Foo<10> {
6    |
-    = note: expected type `11`
-               found type `12`
-               found type `12`
+    = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
- error: aborting due to previous error
+ error: unconstrained generic constant
+   --> $DIR/generic-expr-default-concrete.rs:9:36
+    |
+    |
+ LL | pub fn different_than_default() -> Foo<10> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
- For more information about this error, try `rustc --explain E0308`.
+ error: aborting due to 2 previous errors
+ 
13 
---
To only update this specific test, also pass `--test-args const-generics/defaults/generic-expr-default-concrete.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/generic-expr-default-concrete.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/generic-expr-default-concrete" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/generic-expr-default-concrete/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL | fn no_constraining() -> Foo<10> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/defaults/generic-expr-default-concrete.rs:9:36
   |
   |
LL | pub fn different_than_default() -> Foo<10> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/defaults/generic-expr-default.rs stdout ----
diff of stderr:

7    = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
9 error: unconstrained generic constant
+   --> $DIR/generic-expr-default.rs:9:52
+    |
+    |
+ LL | pub fn has_evaluatable_bound<const N1: usize>() -> Foo<N1> where [(); N1 + 1]: {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
+ error: unconstrained generic constant
10   --> $DIR/generic-expr-default.rs:14:58
11    |
11    |
12 LL | fn needs_evaluatable_bound_alias<T, const N: usize>() -> FooAlias<N>
14    |
14    |
15    = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
- error: aborting due to 2 previous errors
+ error: aborting due to 3 previous errors
18 
19 
---
To only update this specific test, also pass `--test-args const-generics/defaults/generic-expr-default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/generic-expr-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/generic-expr-default" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/generic-expr-default/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL | pub fn needs_evaluatable_bound<const N1: usize>() -> Foo<N1> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/defaults/generic-expr-default.rs:9:52
   |
   |
LL | pub fn has_evaluatable_bound<const N1: usize>() -> Foo<N1> where [(); N1 + 1]: {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/defaults/generic-expr-default.rs:14:58
   |
   |
LL | fn needs_evaluatable_bound_alias<T, const N: usize>() -> FooAlias<N>
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
note: required by a bound in `Foo`
   |
   |
LL | struct Foo<const N: u8>([u8; N as usize])
   |        --- required by a bound in this
LL | where
LL |     [(); N as usize]:;
   |          ^^^^^^^^^^ required by this bound in `Foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs:14:22
   |
   |
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs:14:26
   |
   |
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs:14:13
   |
   |
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
note: required by a bound in `Foo`
   |
   |
LL | struct Foo<const N: u8>([u8; N as usize])
   |        --- required by a bound in this
LL | where
LL |     [(); N as usize]:;
   |          ^^^^^^^^^^ required by this bound in `Foo`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<N, { N as u128 }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<N, { N as u128 }>`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
note: required by a bound in `use_trait_impl::assert_impl`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:12:23
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:14:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<N, { N as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as u128 }`, found `{ O as u128 }`
   |
   = note: expected type `{ N as u128 }`
              found type `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:15:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<N, { N as _ }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<N, { N as _ }>`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
note: required by a bound in `use_trait_impl::assert_impl`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:12:23
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:15:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<N, { N as _ }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as _ }`, found `{ O as u128 }`
   |
   = note: expected type `{ N as _ }`
              found type `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:16:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<12, { 12 as u128 }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<12, { 12 as u128 }>`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
note: required by a bound in `use_trait_impl::assert_impl`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:12:23
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:17:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<13, 13>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<13, 13>`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
note: required by a bound in `use_trait_impl::assert_impl`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:12:23
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:22:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<N, { N as u128 }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<N, { N as u128 }>`
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
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:22:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<N, { N as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as u128 }`, found `{ O as u128 }`
   |
   = note: expected type `{ N as u128 }`
              found type `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:23:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<N, { N as _ }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<N, { N as _ }>`
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
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:23:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<N, { N as _ }>>();
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as _ }`, found `{ O as u128 }`
   |
   = note: expected type `{ N as _ }`
              found type `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:24:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<12, { 12 as u128 }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<12, { 12 as u128 }>`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `use_trait_impl_2::assert_impl`
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-4.rs:25:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<13, 13>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<13, 13>`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `use_trait_impl_2::assert_impl`
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs stdout ----
diff of stderr:

52    = note: expected type `{ N as _ }`
53               found type `{ O as u128 }`
+ error: unconstrained generic constant
+   --> $DIR/abstract-const-as-cast-3.rs:23:5
+    |
+    |
+ LL |     assert_impl::<HasCastInTraitImpl<13, { 12 as u128 }>>();
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
+ note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<13, { 12 as u128 }>`
+    |
+    |
+ LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
+    |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: required by a bound in `use_trait_impl::assert_impl`
+    |
+    |
+ LL |     fn assert_impl<T: Trait>() {}
+    |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
55 error[E0308]: mismatched types
56   --> $DIR/abstract-const-as-cast-3.rs:23:5
57    |


61    = note: expected type `12`
62               found type `13`
63 
+ error: unconstrained generic constant
+   --> $DIR/abstract-const-as-cast-3.rs:25:5
+    |
+ LL |     assert_impl::<HasCastInTraitImpl<14, 13>>();
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
+ note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<14, 13>`
+    |
+    |
+ LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
+    |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: required by a bound in `use_trait_impl::assert_impl`
+    |
+    |
+ LL |     fn assert_impl<T: Trait>() {}
+    |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
64 error[E0308]: mismatched types
65   --> $DIR/abstract-const-as-cast-3.rs:25:5
66    |


124    = note: expected type `{ N as _ }`
125               found type `{ O as u128 }`
+ error: unconstrained generic constant
+   --> $DIR/abstract-const-as-cast-3.rs:41:5
+    |
+    |
+ LL |     assert_impl::<HasCastInTraitImpl<13, { 12 as u128 }>>();
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
+ note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<13, { 12 as u128 }>`
+    |
+    |
+ LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
+    |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: required by a bound in `use_trait_impl_2::assert_impl`
+    |
+    |
+ LL |     fn assert_impl<T: Trait>() {}
+    |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
127 error[E0308]: mismatched types
128   --> $DIR/abstract-const-as-cast-3.rs:41:5
129    |


133    = note: expected type `12`
134               found type `13`
135 
+ error: unconstrained generic constant
+   --> $DIR/abstract-const-as-cast-3.rs:43:5
+    |
+ LL |     assert_impl::<HasCastInTraitImpl<14, 13>>();
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
+ note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<14, 13>`
+    |
+    |
+ LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
+    |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: required by a bound in `use_trait_impl_2::assert_impl`
+    |
+    |
+ LL |     fn assert_impl<T: Trait>() {}
+    |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
136 error[E0308]: mismatched types
137   --> $DIR/abstract-const-as-cast-3.rs:43:5
138    |

---
148 


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
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1 }, { N as u128 }>`
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
   = note: expected type `{ N as u128 }`
              found type `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:20:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as _ }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1 }, { N as _ }>`
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
   = note: expected type `{ N as _ }`
              found type `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:23:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<13, { 12 as u128 }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<13, { 12 as u128 }>`
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
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:23:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<13, { 12 as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `12`, found `13`
   = note: expected type `12`
              found type `13`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:25:5
   |
LL |     assert_impl::<HasCastInTraitImpl<14, 13>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<14, 13>`
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
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:25:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<14, 13>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `13`, found `14`
   = note: expected type `13`
              found type `14`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:35:5
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as u128 }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1 }, { N as u128 }>`
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
   = note: expected type `{ N as u128 }`
              found type `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:38:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as _ }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1 }, { N as _ }>`
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
   = note: expected type `{ N as _ }`
              found type `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:41:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<13, { 12 as u128 }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<13, { 12 as u128 }>`
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
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:41:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<13, { 12 as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `12`, found `13`
   = note: expected type `12`
              found type `13`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:43:5
   |
LL |     assert_impl::<HasCastInTraitImpl<14, 13>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<14, 13>`
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
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:43:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<14, 13>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `13`, found `14`
   = note: expected type `13`
              found type `14`

error: aborting due to 16 previous errors
---
---- [ui] src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     _q = foo::<_, 2>(_q);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<T, const N: usize>(_: T) -> <() as Foo<{ N + 1 }>>::Assoc
   |    --- required by a bound in this
LL | where
LL |     (): Foo<{ N + 1 }>,
   |             ^^^^^^^^^ required by this bound in `foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs:27:10
   |
   |
LL |     _q = foo::<_, 2>(_q);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<T, const N: usize>(_: T) -> <() as Foo<{ N + 1 }>>::Assoc
   |                                               ^^^^^^^^^ required by this bound in `foo`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/associated-consts.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/associated-consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/associated-consts/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/associated-consts/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     test::<FooCipher, 128>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); M - C::BLOCK_SIZE]:`
note: required by a bound in `test`
   |
   |
LL | pub fn test<C: BlockCipher, const M: usize>()
   |        ---- required by a bound in this
LL | where
LL |     [u8; M - C::BLOCK_SIZE]: Sized,
   |          ^^^^^^^^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/associated-consts.rs:30:5
   |
   |
LL |     test::<BarCipher, 64>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); M - C::BLOCK_SIZE]:`
note: required by a bound in `test`
   |
   |
LL | pub fn test<C: BlockCipher, const M: usize>()
   |        ---- required by a bound in this
LL | where
LL |     [u8; M - C::BLOCK_SIZE]: Sized,
   |          ^^^^^^^^^^^^^^^^^ required by this bound in `test`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/division.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/division.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/division/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/division/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
LL |     with_bound::<4>();
   |     ^^^^^^^^^^^^^^^
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N / 2]:`
note: required by a bound in `with_bound`
   |
   |
LL | fn with_bound<const N: usize>() where [u8; N / 2]: Sized {
   |                                            ^^^^^ required by this bound in `with_bound`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/different-fn.rs stdout ----
diff of stderr:

15    |
16    = help: try adding a `where` bound using this expression: `where [(); size_of::<Foo<T>>()]:`
- error: aborting due to 2 previous errors
+ error: unconstrained generic constant
+   --> $DIR/different-fn.rs:16:5
+    |
+    |
+ LL |     test::<u32>();
+    |     ^^^^^^^^^^^
+    |
+    = help: try adding a `where` bound using this expression: `where [(); size_of::<T>()]:`
+ note: required by a bound in `test`
+    |
+    |
+ LL | fn test<T>() -> [u8; size_of::<T>()] {
+    |                      ^^^^^^^^^^^^^^ required by this bound in `test`
+ error: unconstrained generic constant
+   --> $DIR/different-fn.rs:16:5
+    |
+ LL |     test::<u32>();
+ LL |     test::<u32>();
+    |     ^^^^^^^^^^^^^
+    |
+    = help: try adding a `where` bound using this expression: `where [(); size_of::<T>()]:`
+ note: required by a bound in `test`
+    |
+    |
+ LL | fn test<T>() -> [u8; size_of::<T>()] {
+    |                      ^^^^^^^^^^^^^^ required by this bound in `test`
+ error: aborting due to 4 previous errors
19 
20 For more information about this error, try `rustc --explain E0308`.
21 
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/different-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/different-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/different-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/different-fn/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/different-fn.rs:10:5
   |
   |
LL |     [0; size_of::<Foo<T>>()]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ expected `size_of::<T>()`, found `size_of::<Foo<T>>()`
   = note: expected type `size_of::<T>()`
   = note: expected type `size_of::<T>()`
              found type `size_of::<Foo<T>>()`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/different-fn.rs:10:9
   |
   |
LL |     [0; size_of::<Foo<T>>()]
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); size_of::<Foo<T>>()]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/different-fn.rs:16:5
   |
LL |     test::<u32>();
LL |     test::<u32>();
   |     ^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); size_of::<T>()]:`
note: required by a bound in `test`
   |
   |
LL | fn test<T>() -> [u8; size_of::<T>()] {
   |                      ^^^^^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/different-fn.rs:16:5
   |
LL |     test::<u32>();
LL |     test::<u32>();
   |     ^^^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); size_of::<T>()]:`
note: required by a bound in `test`
   |
   |
LL | fn test<T>() -> [u8; size_of::<T>()] {
   |                      ^^^^^^^^^^^^^^ required by this bound in `test`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/elaborate-trait-pred.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/elaborate-trait-pred.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/elaborate-trait-pred/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/elaborate-trait-pred/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
LL | impl Foo for u64 {}
   |      ^^^
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); size_of::<Self>()]:`
note: required by a bound in `Foo`
   |
LL | trait Foo: Sized
   |       --- required by a bound in this
LL | where
LL | where
LL |     [(); size_of::<Self>()]: Sized,
   |          ^^^^^^^^^^^^^^^^^ required by this bound in `Foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/elaborate-trait-pred.rs:15:6
   |
LL | impl Foo for u32 {}
LL | impl Foo for u32 {}
   |      ^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); size_of::<Self>()]:`
note: required by a bound in `Foo`
   |
LL | trait Foo: Sized
   |       --- required by a bound in this
LL | where
LL | where
LL |     [(); size_of::<Self>()]: Sized,
   |          ^^^^^^^^^^^^^^^^^ required by this bound in `Foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/elaborate-trait-pred.rs:17:11
   |
   |
LL | fn foo<T: Foo>() -> [u8; size_of::<T>()] {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); size_of::<Self>()]:`
note: required by a bound in `Foo`
   |
LL | trait Foo: Sized
   |       --- required by a bound in this
LL | where
LL | where
LL |     [(); size_of::<Self>()]: Sized,
   |          ^^^^^^^^^^^^^^^^^ required by this bound in `Foo`
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/eval-privacy.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/eval-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-privacy" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-privacy/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/const-generics/generic_const_exprs/eval-try-unify.rs stdout ----

error: test compilation failed although it shouldn't!
error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/eval-try-unify.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-try-unify" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-try-unify/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(generic_const_exprs)]
   |            ^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/eval-try-unify.rs:23:5
   |
LL |     uses_assoc_type::<u16, N>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + T::ASSOC]:`
note: required by a bound in `uses_assoc_type`
   |
   |
LL | fn uses_assoc_type<T: Generic, const N: usize>() -> [u8; N + T::ASSOC] {
   |                                                          ^^^^^^^^^^^^ required by this bound in `uses_assoc_type`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/eval-try-unify.rs:23:5
   |
   |
LL |     uses_assoc_type::<u16, N>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N + 13`, found `N + T::ASSOC`
   = note: expected type `N + 13`
   = note: expected type `N + 13`
              found type `N + T::ASSOC`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/eval-try-unify.rs:23:5
   |
   |
LL |     uses_assoc_type::<u16, N>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + T::ASSOC]:`
note: required by a bound in `uses_assoc_type`
   |
   |
LL | fn uses_assoc_type<T: Generic, const N: usize>() -> [u8; N + T::ASSOC] {
   |                                                          ^^^^^^^^^^^^ required by this bound in `uses_assoc_type`
error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/from-sig.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/from-sig.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/from-sig/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/from-sig/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let _: Foo<true> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> {
   |                                  ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/from-sig.rs:13:25
   |
   |
LL |     let _: Foo<false> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> {
   |                                  ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/from-sig.rs:12:24
   |
   |
LL |     let _: Foo<true> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> {
   |                                  ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/from-sig.rs:12:24
   |
   |
LL |     let _: Foo<true> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/from-sig.rs:13:25
   |
   |
LL |     let _: Foo<false> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> {
   |                                  ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/from-sig.rs:13:25
   |
   |
LL |     let _: Foo<false> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
error: aborting due to 6 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/from-sig-fail.rs stdout ----
diff of stderr:

- error[E0080]: evaluation of `test::<0>::{constant#0}` failed
+ error: unconstrained generic constant
+    |
+    |
+ LL |     test::<0>();
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
+ note: required by a bound in `test`
3    |
3    |
4 LL | fn test<const N: usize>() -> [u8; N - 1] {
-    |                                   ^^^^^ attempt to compute `0_usize - 1_usize`, which would overflow
+    |                                   ^^^^^ required by this bound in `test`
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: unconstrained generic constant
+   --> $DIR/from-sig-fail.rs:10:5
+    |
+ LL |     test::<0>();
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
+ note: required by a bound in `test`
+    |
+    |
+ LL | fn test<const N: usize>() -> [u8; N - 1] {
+    |                                   ^^^^^ required by this bound in `test`
- For more information about this error, try `rustc --explain E0080`.
+ error: aborting due to 2 previous errors
+ 
10 
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/from-sig-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/from-sig-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/from-sig-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/from-sig-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
LL |     test::<0>();
   |     ^^^^^^^^^
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> [u8; N - 1] {
   |                                   ^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/from-sig-fail.rs:10:5
   |
LL |     test::<0>();
LL |     test::<0>();
   |     ^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> [u8; N - 1] {
   |                                   ^^^^^ required by this bound in `test`
error: aborting due to 2 previous errors
------------------------------------------


---
9 
- error: unconstrained generic constant
-   --> $DIR/issue-62504.rs:18:25
-    |
- LL |         ArrayHolder([0; Self::SIZE])
-    |
-    |
-    = help: try adding a `where` bound using this expression: `where [(); Self::SIZE]:`
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
19 
20 For more information about this error, try `rustc --explain E0308`.
20 For more information about this error, try `rustc --explain E0308`.
21 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full/issue-62504.full.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-62504.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-62504.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-62504.rs:18:21
   |
   |
LL |         ArrayHolder([0; Self::SIZE])
   |                     ^^^^^^^^^^^^^^^ expected `X`, found `Self::SIZE`
   = note: expected type `X`
              found type `Self::SIZE`

error: aborting due to previous error
---
---- [ui] src/test/ui/const-generics/generic_const_exprs/fn_call.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/fn_call/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/fn_call/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     assert_eq!([0; 8], test_simple::<u64>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>()]:`
note: required by a bound in `test_simple`
   |
   |
LL | fn test_simple<T>() -> [u8; std::mem::size_of::<T>()]
   |    ----------- required by a bound in this
LL | where
LL |     [u8; std::mem::size_of::<T>()]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_simple`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:28:24
   |
   |
LL |     assert_eq!([0; 8], test_simple::<u64>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>()]:`
note: required by a bound in `test_simple`
   |
   |
LL | fn test_simple<T>() -> [u8; std::mem::size_of::<T>()]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_simple`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:28:24
   |
   |
LL |     assert_eq!([0; 8], test_simple::<u64>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>()]:`
note: required by a bound in `test_simple`
   |
   |
LL | fn test_simple<T>() -> [u8; std::mem::size_of::<T>()]
   |    ----------- required by a bound in this
LL | where
LL |     [u8; std::mem::size_of::<T>()]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_simple`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:28:24
   |
   |
LL |     assert_eq!([0; 8], test_simple::<u64>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>()]:`
note: required by a bound in `test_simple`
   |
   |
LL | fn test_simple<T>() -> [u8; std::mem::size_of::<T>()]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_simple`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:28:5
   |
   |
LL |     assert_eq!([0; 8], test_simple::<u64>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>()]:`
note: required by a bound in `test_simple`
   |
   |
LL | fn test_simple<T>() -> [u8; std::mem::size_of::<T>()]
   |    ----------- required by a bound in this
LL | where
LL |     [u8; std::mem::size_of::<T>()]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_simple`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:28:5
   |
   |
LL |     assert_eq!([0; 8], test_simple::<u64>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>()]:`
note: required by a bound in `test_simple`
   |
   |
LL | fn test_simple<T>() -> [u8; std::mem::size_of::<T>()]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_simple`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:28:5
   |
   |
LL |     assert_eq!([0; 8], test_simple::<u64>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>()]:`
note: required by a bound in `test_simple`
   |
   |
LL | fn test_simple<T>() -> [u8; std::mem::size_of::<T>()]
   |    ----------- required by a bound in this
LL | where
LL |     [u8; std::mem::size_of::<T>()]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_simple`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:28:5
   |
   |
LL |     assert_eq!([0; 8], test_simple::<u64>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>()]:`
note: required by a bound in `test_simple`
   |
   |
LL | fn test_simple<T>() -> [u8; std::mem::size_of::<T>()]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_simple`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:29:25
   |
   |
LL |     assert_eq!([0; 12], test_with_args::<u64, 4>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); test_me::<T>(N, N + 1) + N]:`
note: required by a bound in `test_with_args`
   |
   |
LL | fn test_with_args<T, const N: usize>() -> [u8; test_me::<T>(N, N + 1) + N]
   |    -------------- required by a bound in this
LL | where
LL |     [u8; test_me::<T>(N, N + 1) + N]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_with_args`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:29:25
   |
   |
LL |     assert_eq!([0; 12], test_with_args::<u64, 4>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); test_me::<T>(N, N + 1) + N]:`
note: required by a bound in `test_with_args`
   |
   |
LL | fn test_with_args<T, const N: usize>() -> [u8; test_me::<T>(N, N + 1) + N]
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_with_args`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:29:25
   |
   |
LL |     assert_eq!([0; 12], test_with_args::<u64, 4>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); test_me::<T>(N, N + 1) + N]:`
note: required by a bound in `test_with_args`
   |
   |
LL | fn test_with_args<T, const N: usize>() -> [u8; test_me::<T>(N, N + 1) + N]
   |    -------------- required by a bound in this
LL | where
LL |     [u8; test_me::<T>(N, N + 1) + N]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_with_args`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:29:25
   |
   |
LL |     assert_eq!([0; 12], test_with_args::<u64, 4>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); test_me::<T>(N, N + 1) + N]:`
note: required by a bound in `test_with_args`
   |
   |
LL | fn test_with_args<T, const N: usize>() -> [u8; test_me::<T>(N, N + 1) + N]
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_with_args`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:29:5
   |
   |
LL |     assert_eq!([0; 12], test_with_args::<u64, 4>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); test_me::<T>(N, N + 1) + N]:`
note: required by a bound in `test_with_args`
   |
   |
LL | fn test_with_args<T, const N: usize>() -> [u8; test_me::<T>(N, N + 1) + N]
   |    -------------- required by a bound in this
LL | where
LL |     [u8; test_me::<T>(N, N + 1) + N]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_with_args`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:29:5
   |
   |
LL |     assert_eq!([0; 12], test_with_args::<u64, 4>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); test_me::<T>(N, N + 1) + N]:`
note: required by a bound in `test_with_args`
   |
   |
LL | fn test_with_args<T, const N: usize>() -> [u8; test_me::<T>(N, N + 1) + N]
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_with_args`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:29:5
   |
   |
LL |     assert_eq!([0; 12], test_with_args::<u64, 4>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); test_me::<T>(N, N + 1) + N]:`
note: required by a bound in `test_with_args`
   |
   |
LL | fn test_with_args<T, const N: usize>() -> [u8; test_me::<T>(N, N + 1) + N]
   |    -------------- required by a bound in this
LL | where
LL |     [u8; test_me::<T>(N, N + 1) + N]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_with_args`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:29:5
   |
   |
LL |     assert_eq!([0; 12], test_with_args::<u64, 4>());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); test_me::<T>(N, N + 1) + N]:`
note: required by a bound in `test_with_args`
   |
   |
LL | fn test_with_args<T, const N: usize>() -> [u8; test_me::<T>(N, N + 1) + N]
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_with_args`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 16 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/cross_crate.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/cross_crate/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/cross_crate/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     assert_eq!(const_evaluatable_lib::test1::<u32>(), [0; 3]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate.rs:12:16
   |
   |
LL |     assert_eq!(const_evaluatable_lib::test1::<u32>(), [0; 3]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate.rs:12:16
   |
   |
LL |     assert_eq!(const_evaluatable_lib::test1::<u32>(), [0; 3]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate.rs:12:16
   |
   |
LL |     assert_eq!(const_evaluatable_lib::test1::<u32>(), [0; 3]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate.rs:12:5
   |
   |
LL |     assert_eq!(const_evaluatable_lib::test1::<u32>(), [0; 3]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate.rs:12:5
   |
   |
LL |     assert_eq!(const_evaluatable_lib::test1::<u32>(), [0; 3]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate.rs:12:5
   |
   |
LL |     assert_eq!(const_evaluatable_lib::test1::<u32>(), [0; 3]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate.rs:12:5
   |
   |
LL |     assert_eq!(const_evaluatable_lib::test1::<u32>(), [0; 3]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate.rs:13:5
   |
LL |     user::<u32>();
LL |     user::<u32>();
   |     ^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `user`
   |
   |
LL | fn user<T>() where [u8; std::mem::size_of::<T>() - 1]: Sized {
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `user`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate.rs:14:5
   |
LL |     user::<u64>();
LL |     user::<u64>();
   |     ^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `user`
   |
   |
LL | fn user<T>() where [u8; std::mem::size_of::<T>() - 1]: Sized {
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `user`
error: aborting due to 10 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/infer-too-generic/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/infer-too-generic/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let (head, tail) = split_first(arr);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `split_first`
   |
   |
LL | fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; N - 1])
   |    ----------- required by a bound in this
LL | where
LL |     [T; N - 1]: Sized,
   |         ^^^^^ required by this bound in `split_first`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs:21:24
   |
   |
LL |     let (head, tail) = split_first(arr);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `split_first`
   |
   |
LL | fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; N - 1])
   |                                                           ^^^^^ required by this bound in `split_first`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs:21:24
   |
   |
LL |     let (head, tail) = split_first(arr);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `split_first`
   |
   |
LL | fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; N - 1])
   |    ----------- required by a bound in this
LL | where
LL |     [T; N - 1]: Sized,
   |         ^^^^^ required by this bound in `split_first`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs:21:24
   |
   |
LL |     let (head, tail) = split_first(arr);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `split_first`
   |
   |
LL | fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; N - 1])
   |                                                           ^^^^^ required by this bound in `split_first`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs:23:5
   |
   |
LL |     assert_eq!(tail, [1, 2, 3, 4]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `split_first`
   |
   |
LL | fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; N - 1])
   |    ----------- required by a bound in this
LL | where
LL |     [T; N - 1]: Sized,
   |         ^^^^^ required by this bound in `split_first`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs:23:5
   |
   |
LL |     assert_eq!(tail, [1, 2, 3, 4]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `split_first`
   |
   |
LL | fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; N - 1])
   |                                                           ^^^^^ required by this bound in `split_first`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs:23:5
   |
   |
LL |     assert_eq!(tail, [1, 2, 3, 4]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `split_first`
---
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.full/issue-72819-generic-in-const-eval.full.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.full/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); {N < usize::MAX / 2}]:`
note: required by a bound in `Arr`
   |
   |
LL | struct Arr<const N: usize>
   |        --- required by a bound in this
LL | where Assert::<{N < usize::MAX / 2}>: IsTrue,
   |                ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Arr`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.rs:20:12
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); {N < usize::MAX / 2}]:`
note: required by a bound in `Arr`
   |
   |
LL | struct Arr<const N: usize>
   |        --- required by a bound in this
LL | where Assert::<{N < usize::MAX / 2}>: IsTrue,
   |                ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Arr`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.rs:20:12
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |            ^^^^^^^^^^^^^^^^^ expected `false`, found `true`
   = note: expected type `false`
              found type `true`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-72819-generic-in-const-eval.rs:20:32
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |                                ^^^ expected `false`, found `true`
   = note: expected type `false`
              found type `true`

error: aborting due to 4 previous errors
---
---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-73899.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-73899.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-73899/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-73899/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
LL |     foo([]);
   |     --- ^^
   |     |
   |     |
   |     required by a bound introduced by this call
   |
   = help: try adding a `where` bound using this expression: `where [(); { N == 0 }]:`
note: required because of the requirements on the impl of `Foo` for `[(); 0]`
   |
   |
LL | impl<const N: usize> Foo for [(); N] where Self: FooImpl<{ N == 0 }> {}
note: required by a bound in `foo`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-73899.rs:15:11
   |
   |
LL | fn foo<T: Foo>(_v: T) {}
   |           ^^^ required by this bound in `foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-73899.rs:19:9
   |
   |
LL |     foo([()]);
   |     |
   |     required by a bound introduced by this call
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N == 0 }]:`
note: required because of the requirements on the impl of `Foo` for `[(); 1]`
   |
   |
LL | impl<const N: usize> Foo for [(); N] where Self: FooImpl<{ N == 0 }> {}
note: required by a bound in `foo`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-73899.rs:15:11
   |
   |
LL | fn foo<T: Foo>(_v: T) {}
   |           ^^^ required by this bound in `foo`
error: aborting due to 2 previous errors
------------------------------------------


---
-    |
- LL |         self.reference.size()
-    |                        ^^^^
-    |
-    = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
- note: required by a bound in `TensorSize::size`
-   --> $DIR/issue-83765.rs:9:31
-    |
- LL |     fn size(&self) -> [usize; Self::DIM];
-    |                               ^^^^^^^^^ required by this bound in `TensorSize::size`
23 error[E0308]: mismatched types
24   --> $DIR/issue-83765.rs:32:9
25    |

---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-83765.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-83765.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-83765" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-83765/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: method not compatible with trait
   |
   |
LL |     fn size(&self) -> [usize; DIM] {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-83765.rs:32:9
   |
LL |         self.reference.size()
   |         ^^^^^^^^^^^^^^^^^^^^^ expected `DIM`, found `Self::DIM`
   = note: expected type `DIM`
              found type `Self::DIM`

error: aborting due to 2 previous errors
---

---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-76595.rs stdout ----
diff of stderr:

16 LL |     test::<2, P>();
18 
- error: aborting due to previous error
+ error: unconstrained generic constant
+   --> $DIR/issue-76595.rs:15:5
+   --> $DIR/issue-76595.rs:15:5
+    |
+ LL |     test::<2>();
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); {core::mem::size_of::<T>() > 4}]:`
+ note: required by a bound in `test`
+    |
+    |
+ LL | fn test<T, const P: usize>() where Bool<{core::mem::size_of::<T>() > 4}>: True {
+    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test`
+ error: aborting due to 2 previous errors
20 
21 For more information about this error, try `rustc --explain E0107`.
22 
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-76595.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-76595.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-76595" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-76595/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0107]: this function takes 2 generic arguments but 1 generic argument was supplied
   |
LL |     test::<2>();
LL |     test::<2>();
   |     ^^^^   - supplied 1 generic argument
   |     expected 2 generic arguments
   |
   |
note: function defined here, with 2 generic parameters: `T`, `P`
   |
   |
LL | fn test<T, const P: usize>() where Bool<{core::mem::size_of::<T>() > 4}>: True {
   |    ^^^^ -  --------------
help: add missing generic argument
   |
LL |     test::<2, P>();

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-76595.rs:15:5
   |
   |
LL |     test::<2>();
   |     ^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); {core::mem::size_of::<T>() > 4}]:`
note: required by a bound in `test`
   |
   |
LL | fn test<T, const P: usize>() where Bool<{core::mem::size_of::<T>() > 4}>: True {
   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0107`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-80742.rs stdout ----
diff of stderr:

- error[E0080]: evaluation of `Inline::<dyn std::fmt::Debug>::{constant#0}` failed
-   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
- LL |     intrinsics::size_of::<T>()
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |     |
-    |     |
-    |     size_of called on unsized type `dyn Debug`
-    |     inside `std::mem::size_of::<dyn Debug>` at $SRC_DIR/core/src/mem/mod.rs:LL:COL
-   ::: $DIR/issue-80742.rs:22:10
-    |
-    |
- LL |     [u8; size_of::<T>() + 1]: ,
-    |          -------------- inside `Inline::<dyn Debug>::{constant#0}` at $DIR/issue-80742.rs:22:10
- 
15 error[E0599]: the function or associated item `new` exists for struct `Inline<dyn Debug>`, but its trait bounds were not satisfied
17    |

29    = note: the following trait bounds were not satisfied:
29    = note: the following trait bounds were not satisfied:
30            `dyn Debug: Sized`
31 
- error[E0080]: evaluation of `Inline::<dyn std::fmt::Debug>::{constant#0}` failed
-   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
- LL |     intrinsics::size_of::<T>()
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |     |
-    |     |
-    |     size_of called on unsized type `dyn Debug`
-    |     inside `std::mem::size_of::<dyn Debug>` at $SRC_DIR/core/src/mem/mod.rs:LL:COL
-   ::: $DIR/issue-80742.rs:14:10
-    |
-    |
- LL |     [u8; size_of::<T>() + 1]: ,
-    |          -------------- inside `Inline::<dyn Debug>::{constant#0}` at $DIR/issue-80742.rs:14:10
46 error[E0277]: the size for values of type `dyn Debug` cannot be known at compilation time
47   --> $DIR/issue-80742.rs:30:15
48    |


60 LL | struct Inline<T: ?Sized>
62 
- error: aborting due to 4 previous errors
+ error: unconstrained generic constant
+   --> $DIR/issue-80742.rs:30:15
+   --> $DIR/issue-80742.rs:30:15
+    |
+ LL |     let dst = Inline::<dyn Debug>::new(0);
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); size_of::<T>() + 1]:`
+ note: required by a bound in `Inline`
+    |
+    |
+ LL | struct Inline<T>
+    |        ------ required by a bound in this
+ LL | where
+ LL |     [u8; size_of::<T>() + 1]: ,
+    |          ^^^^^^^^^^^^^^^^^^ required by this bound in `Inline`
- Some errors have detailed explanations: E0080, E0277, E0599.
- For more information about an error, try `rustc --explain E0080`.
+ error: aborting due to 3 previous errors
+ 
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-80742.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80742" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80742/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the function or associated item `new` exists for struct `Inline<dyn Debug>`, but its trait bounds were not satisfied
   |
LL | struct Inline<T>
   | ---------------- function or associated item `new` not found for this struct
...
...
LL |     let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |                                    ^^^ function or associated item cannot be called on `Inline<dyn Debug>` due to unsatisfied trait bounds
  ::: /checkout/library/core/src/fmt/mod.rs:658:1
   |
LL | pub trait Debug {
LL | pub trait Debug {
   | --------------- doesn't satisfy `dyn Debug: Sized`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `dyn Debug: Sized`
error[E0277]: the size for values of type `dyn Debug` cannot be known at compilation time
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:30:15
   |
   |
LL |     let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |
   = help: the trait `Sized` is not implemented for `dyn Debug`
note: required by a bound in `Inline`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:12:15
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:12:15
   |
LL | struct Inline<T>
   |               ^ required by this bound in `Inline`
help: consider relaxing the implicit `Sized` restriction
   |
LL | struct Inline<T: ?Sized>

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:30:15
   |
   |
LL |     let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); size_of::<T>() + 1]:`
note: required by a bound in `Inline`
   |
LL | struct Inline<T>
   |        ------ required by a bound in this
LL | where
LL | where
LL |     [u8; size_of::<T>() + 1]: ,
   |          ^^^^^^^^^^^^^^^^^^ required by this bound in `Inline`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/less_than.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/less_than/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/less_than/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let _: Foo<true> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |                                                        ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:12:24
   |
   |
LL |     let _: Foo<true> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |                                  ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:13:25
   |
   |
LL |     let _: Foo<false> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |                                                        ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:13:25
   |
   |
LL |     let _: Foo<false> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |                                  ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:12:24
   |
   |
LL |     let _: Foo<true> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |                                                        ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:12:24
   |
   |
LL |     let _: Foo<true> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |                                  ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:12:24
   |
   |
LL |     let _: Foo<true> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:13:25
   |
   |
LL |     let _: Foo<false> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |                                                        ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:13:25
   |
   |
LL |     let _: Foo<false> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |                                  ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:13:25
   |
   |
LL |     let _: Foo<false> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
error: aborting due to 10 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL | fn substs2<const M: usize>() -> Substs1<{ M * 2 }>
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `Substs1`
   |
   |
LL | struct Substs1<const N: usize>([u8; N + 1])
   |        ------- required by a bound in this
LL | where
LL |     [(); N + 1]: ;
   |          ^^^^^ required by this bound in `Substs1`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs:22:33
   |
   |
LL | fn substs3<const L: usize>() -> Substs1<{ (L - 1) * 2 }>
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `Substs1`
   |
   |
LL | struct Substs1<const N: usize>([u8; N + 1])
   |        ------- required by a bound in this
LL | where
LL |     [(); N + 1]: ;
   |          ^^^^^ required by this bound in `Substs1`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/let-bindings.rs stdout ----
diff of stderr:

2   --> $DIR/let-bindings.rs:6:68
3    |
4 LL | fn test<const N: usize>() -> [u8; { let x = N; N + 1 }] where [u8; { let x = N; N + 1 }]: Default {
-    |                                                                    ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                                                    ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
6    |
6    |
7    = help: consider moving this anonymous constant into a `const` function

11   --> $DIR/let-bindings.rs:6:35
12    |
12    |
13 LL | fn test<const N: usize>() -> [u8; { let x = N; N + 1 }] where [u8; { let x = N; N + 1 }]: Default {
-    |                                   ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                   ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
15    |
15    |
16    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/let-bindings/let-bindings.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/let-bindings/let-bindings.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/let-bindings.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/let-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/let-bindings" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/let-bindings/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL | fn test<const N: usize>() -> [u8; { let x = N; N + 1 }] where [u8; { let x = N; N + 1 }]: Default {
   |                                                                    ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/let-bindings.rs:6:35
   |
   |
LL | fn test<const N: usize>() -> [u8; { let x = N; N + 1 }] where [u8; { let x = N; N + 1 }]: Default {
   |                                   ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/nested-abstract-consts-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/nested-abstract-consts-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested-abstract-consts-1/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested-abstract-consts-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     callee::<{ N1 + 1 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); M2 + 1]:`
note: required by a bound in `callee`
   |
   |
LL | fn callee<const M2: usize>() -> usize
   |    ------ required by a bound in this
LL | where
LL |     [u8; M2 + 1]: Sized,
   |          ^^^^^^ required by this bound in `callee`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/nested-abstract-consts-1.rs:21:16
   |
   |
LL |     assert_eq!(caller::<4>(), 5);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N1 + 1]:`
note: required by a bound in `caller`
   |
   |
LL | fn caller<const N1: usize>() -> usize
   |    ------ required by a bound in this
LL | where
LL |     [u8; N1 + 1]: Sized,
   |          ^^^^^^ required by this bound in `caller`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/nested-abstract-consts-1.rs:21:16
   |
   |
LL |     assert_eq!(caller::<4>(), 5);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); (N1 + 1) + 1]:`
note: required by a bound in `caller`
   |
   |
LL | fn caller<const N1: usize>() -> usize
   |    ------ required by a bound in this
...
LL |     [u8; (N1 + 1) + 1]: Sized,
   |          ^^^^^^^^^^^^ required by this bound in `caller`
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/no_dependence.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/no_dependence.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/no_dependence" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/no_dependence/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |      two_args::<N, 2>() // no lint
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); M + 2]:`
note: required by a bound in `two_args`
   |
   |
LL | fn two_args<const N: usize, const M: usize>() -> [u8; M + 2] {
   |                                                       ^^^^^ required by this bound in `two_args`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/no_dependence.rs:10:6
   |
   |
LL |      two_args::<N, 2>() // no lint
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); M + 2]:`
note: required by a bound in `two_args`
   |
   |
LL | fn two_args<const N: usize, const M: usize>() -> [u8; M + 2] {
   |                                                       ^^^^^ required by this bound in `two_args`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/nested-abstract-consts-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/nested-abstract-consts-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested-abstract-consts-2/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested-abstract-consts-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `bar` found for struct `Generic<10>` in the current scope
   |
   |
LL | struct Generic<const K: u64>;
   | ---------------------------- method `bar` not found for this struct
...
LL |     assert_eq!((Generic::<10>).bar(), 11);
   |                                ^^^ method cannot be called on `Generic<10>` due to unsatisfied trait bounds
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds.rs stdout ----
diff of stderr:

20    |        ^^^^ ...because method `test` references the `Self` type in its `where` clause
21    = help: consider moving `test` to another trait
- error: aborting due to previous error
+ error: unconstrained generic constant
+   --> $DIR/object-safety-err-where-bounds.rs:19:7
+    |
+    |
+ LL |     v.test();
+    |       ^^^^
+    |
+    = help: try adding a `where` bound using this expression: `where [(); bar::<Self>()]:`
+ note: required by a bound in `Foo::test`
+    |
+    |
+ LL |     fn test(&self) where [u8; bar::<Self>()]: Sized;
+    |                               ^^^^^^^^^^^^^ required by this bound in `Foo::test`
+ error: aborting due to 2 previous errors
24 
25 

---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/object-safety-err-where-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds.rs:9:8
   |
   |
LL |     fn test(&self) where [u8; bar::<Self>()]: Sized;
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds.rs:3:9
   |
   |
LL | #![deny(where_clauses_object_safety)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #51443 <https://github.com/rust-lang/rust/issues/51443>
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds.rs:9:8
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds.rs:9:8
   |
LL | trait Foo {
   |       --- this trait cannot be made into an object...
LL |     fn test(&self) where [u8; bar::<Self>()]: Sized;
   |        ^^^^ ...because method `test` references the `Self` type in its `where` clause
   = help: consider moving `test` to another trait
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds.rs:19:7
   |
LL |     v.test();
LL |     v.test();
   |       ^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); bar::<Self>()]:`
note: required by a bound in `Foo::test`
   |
   |
LL |     fn test(&self) where [u8; bar::<Self>()]: Sized;
   |                               ^^^^^^^^^^^^^ required by this bound in `Foo::test`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err.rs stdout ----
diff of stderr:

- error[E0284]: type annotations needed
+ error: unconstrained generic constant
+   --> $DIR/object-safety-ok-infer-err.rs:9:28
+    |
+ LL |     fn test(&self) -> [u8; N + 1] {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
+ note: the requirement `the constant `<() as Foo<N>>::test::{constant#0}` can be evaluated` appears on the `impl`'s method `test` but not on the corresponding trait's method
+    |
+    |
+ LL | trait Foo<const N: usize> {
+    |       --- in this trait
+ LL |     fn test(&self) -> [u8; N + 1];
+    |        ^^^^ this trait's method doesn't have the requirement `the constant `<() as Foo<N>>::test::{constant#0}` can be evaluated`
+ error[E0308]: method not compatible with trait
+   --> $DIR/object-safety-ok-infer-err.rs:9:5
+    |
+    |
+ LL |     fn test(&self) -> [u8; N + 1] {
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N + 1`, found `N + 1`
+    = note: expected type `N + 1`
+               found type `N + 1`
+ 
+ error: unconstrained generic constant
+ error: unconstrained generic constant
+   --> $DIR/object-safety-ok-infer-err.rs:15:18
+    |
+ LL |     assert_eq!(v.test(), [0; N + 1]);
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
+ note: required by a bound in `Foo::test`
+    |
+    |
+ LL |     fn test(&self) -> [u8; N + 1];
+    |                            ^^^^^ required by this bound in `Foo::test`
+ error: unconstrained generic constant
+   --> $DIR/object-safety-ok-infer-err.rs:15:5
+    |
+    |
+ LL |     assert_eq!(v.test(), [0; N + 1]);
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
+ note: required by a bound in `Foo::test`
+    |
+    |
+ LL |     fn test(&self) -> [u8; N + 1];
+    |                            ^^^^^ required by this bound in `Foo::test`
+    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error[E0308]: mismatched types
+   --> $DIR/object-safety-ok-infer-err.rs:15:5
+    |
+    |
+ LL |     assert_eq!(v.test(), [0; N + 1]);
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N + 1`, found `N + 1`
+    = note: expected type `N + 1`
+               found type `N + 1`
+               found type `N + 1`
+    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error: unconstrained generic constant
+   --> $DIR/object-safety-ok-infer-err.rs:15:5
+    |
+    |
+ LL |     assert_eq!(v.test(), [0; N + 1]);
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
+ note: required by a bound in `Foo::test`
+    |
+    |
+ LL |     fn test(&self) -> [u8; N + 1];
+    |                            ^^^^^ required by this bound in `Foo::test`
+    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error: unconstrained generic constant
2   --> $DIR/object-safety-ok-infer-err.rs:19:5
3    |
3    |
4 LL |     use_dyn(&());

-    |     ^^^^^^^ cannot infer the value of the const parameter `N` declared on the function `use_dyn`
6    |
6    |
+    = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
7 note: required by a bound in `use_dyn`
9    |


10 LL | fn use_dyn<const N: usize>(v: &dyn Foo<N>) where [u8; N + 1]: Sized {
11    |                                                       ^^^^^ required by this bound in `use_dyn`
- help: consider specifying the generic argument
-    |
- LL |     use_dyn::<N>(&());
16 
- error: aborting due to previous error
+ error: aborting due to 7 previous errors
18 
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/object-safety-ok-infer-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     fn test(&self) -> [u8; N + 1] {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: the requirement `the constant `<() as Foo<N>>::test::{constant#0}` can be evaluated` appears on the `impl`'s method `test` but not on the corresponding trait's method
   |
   |
LL | trait Foo<const N: usize> {
   |       --- in this trait
LL |     fn test(&self) -> [u8; N + 1];
   |        ^^^^ this trait's method doesn't have the requirement `the constant `<() as Foo<N>>::test::{constant#0}` can be evaluated`
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err.rs:9:5
   |
   |
LL |     fn test(&self) -> [u8; N + 1] {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N + 1`, found `N + 1`
   = note: expected type `N + 1`
              found type `N + 1`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err.rs:15:18
   |
LL |     assert_eq!(v.test(), [0; N + 1]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `Foo::test`
   |
   |
LL |     fn test(&self) -> [u8; N + 1];
   |                            ^^^^^ required by this bound in `Foo::test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err.rs:15:5
   |
   |
LL |     assert_eq!(v.test(), [0; N + 1]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `Foo::test`
   |
   |
LL |     fn test(&self) -> [u8; N + 1];
   |                            ^^^^^ required by this bound in `Foo::test`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err.rs:15:5
   |
   |
LL |     assert_eq!(v.test(), [0; N + 1]);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N + 1`, found `N + 1`
   = note: expected type `N + 1`
              found type `N + 1`
              found type `N + 1`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err.rs:15:5
   |
   |
LL |     assert_eq!(v.test(), [0; N + 1]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `Foo::test`
   |
   |
LL |     fn test(&self) -> [u8; N + 1];
   |                            ^^^^^ required by this bound in `Foo::test`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err.rs:19:5
   |
   |
LL |     use_dyn(&());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `use_dyn`
   |
   |
LL | fn use_dyn<const N: usize>(v: &dyn Foo<N>) where [u8; N + 1]: Sized {
   |                                                       ^^^^^ required by this bound in `use_dyn`
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/simple_fail.rs stdout ----
diff of stderr:

- error[E0080]: evaluation of `test::<0>::{constant#0}` failed
+ error: unconstrained generic constant
+    |
+    |
+ LL |     test::<0>();
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
+ note: required by a bound in `test`
3    |
3    |
+ LL | fn test<const N: usize>() -> Arr<N>
+    |    ---- required by a bound in this
+ LL | where
4 LL |     [u8; N - 1]: Sized,
-    |          ^^^^^ attempt to compute `0_usize - 1_usize`, which would overflow
+    |          ^^^^^ required by this bound in `test`
6 
- error[E0080]: evaluation of `Arr::<0>::{constant#0}` failed
-   --> $DIR/simple_fail.rs:4:33
+ error: unconstrained generic constant
9    |
9    |
- LL | type Arr<const N: usize> = [u8; N - 1];
-    |                                 ^^^^^ attempt to compute `0_usize - 1_usize`, which would overflow
+ LL |     test::<0>();
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
13 error: aborting due to 2 previous errors
14 

- For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/simple_fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/simple_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/simple_fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/simple_fail/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
LL |     test::<0>();
   |     ^^^^^^^^^
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Arr<N>
   |    ---- required by a bound in this
LL | where
LL |     [u8; N - 1]: Sized,
   |          ^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/simple_fail.rs:16:5
   |
LL |     test::<0>();
LL |     test::<0>();
   |     ^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/object-safety-ok.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-ok/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-ok/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     fn test(&self) -> [u8; N + 1] {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: the requirement `the constant `<() as Foo<N>>::test::{constant#0}` can be evaluated` appears on the `impl`'s method `test` but not on the corresponding trait's method
   |
   |
LL | trait Foo<const N: usize> {
   |       --- in this trait
LL |     fn test(&self) -> [u8; N + 1];
   |        ^^^^ this trait's method doesn't have the requirement `the constant `<() as Foo<N>>::test::{constant#0}` can be evaluated`
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok.rs:10:5
   |
   |
LL |     fn test(&self) -> [u8; N + 1] {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N + 1`, found `N + 1`
   = note: expected type `N + 1`
              found type `N + 1`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok.rs:16:18
   |
LL |     assert_eq!(v.test(), [0; N + 1]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `Foo::test`
   |
   |
LL |     fn test(&self) -> [u8; N + 1];
   |                            ^^^^^ required by this bound in `Foo::test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok.rs:16:5
   |
   |
LL |     assert_eq!(v.test(), [0; N + 1]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `Foo::test`
   |
   |
LL |     fn test(&self) -> [u8; N + 1];
   |                            ^^^^^ required by this bound in `Foo::test`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok.rs:16:5
   |
   |
LL |     assert_eq!(v.test(), [0; N + 1]);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N + 1`, found `N + 1`
   = note: expected type `N + 1`
              found type `N + 1`
              found type `N + 1`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok.rs:16:5
   |
   |
LL |     assert_eq!(v.test(), [0; N + 1]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `Foo::test`
   |
   |
LL |     fn test(&self) -> [u8; N + 1];
   |                            ^^^^^ required by this bound in `Foo::test`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok.rs:20:5
   |
   |
LL |     use_dyn::<3>(&());
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `use_dyn`
   |
   |
LL | fn use_dyn<const N: usize>(v: &dyn Foo<N>) where [u8; N + 1]: Sized {
   |                                                       ^^^^^ required by this bound in `use_dyn`
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/unused_expr.rs stdout ----
diff of stderr:

2   --> $DIR/unused_expr.rs:4:34
3    |
4 LL | fn add<const N: usize>() -> [u8; { N + 1; 5 }] {
-    |                                  ^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                  ^^^^^^^^^^^^ blocks are not supported in generic
6    |
6    |
7    = help: consider moving this anonymous constant into a `const` function

11   --> $DIR/unused_expr.rs:9:34
12    |
12    |
13 LL | fn div<const N: usize>() -> [u8; { N / 1; 5 }] {
-    |                                  ^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                  ^^^^^^^^^^^^ blocks are not supported in generic
15    |
15    |
16    = help: consider moving this anonymous constant into a `const` function

20   --> $DIR/unused_expr.rs:16:38
21    |
21    |
22 LL | fn fn_call<const N: usize>() -> [u8; { foo(N); 5 }] {
-    |                                      ^^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                      ^^^^^^^^^^^^^ blocks are not supported in generic
24    |
24    |
25    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unused_expr/unused_expr.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/unused_expr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/unused_expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unused_expr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unused_expr/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL | fn add<const N: usize>() -> [u8; { N + 1; 5 }] {
   |                                  ^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unused_expr.rs:9:34
   |
   |
LL | fn div<const N: usize>() -> [u8; { N / 1; 5 }] {
   |                                  ^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unused_expr.rs:16:38
   |
   |
LL | fn fn_call<const N: usize>() -> [u8; { foo(N); 5 }] {
   |                                      ^^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     make_array::<{ N * 2 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); M + 1]:`
note: required by a bound in `make_array`
   |
   |
LL | fn make_array<const M: usize>() -> [(); M + 1] {
   |                                         ^^^^^ required by this bound in `make_array`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs:10:18
   |
   |
LL |     make_array::<{ N * 2 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N * 2 }]:`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs:10:5
   |
   |
LL |     make_array::<{ N * 2 }>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `(N * 2) + 1`, found `M + 1`
   |
   = note: expected type `(N * 2) + 1`
              found type `M + 1`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs:10:5
   |
   |
LL |     make_array::<{ N * 2 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); M + 1]:`
note: required by a bound in `make_array`
   |
   |
LL | fn make_array<const M: usize>() -> [(); M + 1] {
   |                                         ^^^^^ required by this bound in `make_array`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs:10:5
   |
   |
LL |     make_array::<{ N * 2 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N * 2 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs:14:16
   |
   |
LL |     assert_eq!(foo::<10>(), [(); 10 * 2 + 1])
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); (N * 2) + 1]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<const N: usize>() -> [(); (N * 2) + 1] {
   |                                  ^^^^^^^^^^^ required by this bound in `foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs:14:16
   |
   |
LL |     assert_eq!(foo::<10>(), [(); 10 * 2 + 1])
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); (N * 2) + 1]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<const N: usize>() -> [(); (N * 2) + 1] {
   |                                  ^^^^^^^^^^^ required by this bound in `foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs:14:5
   |
   |
LL |     assert_eq!(foo::<10>(), [(); 10 * 2 + 1])
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); (N * 2) + 1]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<const N: usize>() -> [(); (N * 2) + 1] {
   |                                  ^^^^^^^^^^^ required by this bound in `foo`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs:14:5
   |
   |
LL |     assert_eq!(foo::<10>(), [(); 10 * 2 + 1])
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); (N * 2) + 1]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<const N: usize>() -> [(); (N * 2) + 1] {
   |                                  ^^^^^^^^^^^ required by this bound in `foo`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/unop.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/unop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unop/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unop/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let _: Foo<false> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { !(N > 10) }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ !(N > 10) }> where Foo<{ !(N > 10) }>: Sized {
   |                                                           ^^^^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unop.rs:12:25
   |
   |
LL |     let _: Foo<false> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { !(N > 10) }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ !(N > 10) }> where Foo<{ !(N > 10) }>: Sized {
   |                                  ^^^^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unop.rs:13:24
   |
   |
LL |     let _: Foo<true> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { !(N > 10) }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ !(N > 10) }> where Foo<{ !(N > 10) }>: Sized {
   |                                                           ^^^^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unop.rs:13:24
   |
   |
LL |     let _: Foo<true> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { !(N > 10) }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ !(N > 10) }> where Foo<{ !(N > 10) }>: Sized {
   |                                  ^^^^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unop.rs:12:25
   |
   |
LL |     let _: Foo<false> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { !(N > 10) }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ !(N > 10) }> where Foo<{ !(N > 10) }>: Sized {
   |                                                           ^^^^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unop.rs:12:25
   |
   |
LL |     let _: Foo<false> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { !(N > 10) }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ !(N > 10) }> where Foo<{ !(N > 10) }>: Sized {
   |                                  ^^^^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unop.rs:12:25
   |
   |
LL |     let _: Foo<false> = test::<12>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { !(N > 10) }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unop.rs:13:24
   |
   |
LL |     let _: Foo<true> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { !(N > 10) }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ !(N > 10) }> where Foo<{ !(N > 10) }>: Sized {
   |                                                           ^^^^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unop.rs:13:24
   |
   |
LL |     let _: Foo<true> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { !(N > 10) }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ !(N > 10) }> where Foo<{ !(N > 10) }>: Sized {
   |                                  ^^^^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unop.rs:13:24
   |
   |
LL |     let _: Foo<true> = test::<9>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { !(N > 10) }]:`
error: aborting due to 10 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/ty-alias-substitution.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/ty-alias-substitution.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/ty-alias-substitution" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/ty-alias-substitution/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL | fn foo<const M: usize>() -> Alias<u32, M>  where [u8; M + 1]: Sized {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issue-97007.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-97007.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-97007" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-97007/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let _ = Walk::new().proceed_to::<1>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { walk(REMAINING, CURRENT, NEXT) }]:`
note: required by a bound in `Walk::<CURRENT, REMAINING>::proceed_to`
   |
   |
LL |         pub fn proceed_to<const NEXT: usize>(
---
-    | |_____^ blocks are not supported in generic constant
+    | |_____^ blocks are not supported in generic
+             constant
11    |
12    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-3.full/issue-67945-3.full.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-67945-3.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67945-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-3.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-3.full/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL |       A: [(); { //[full]~ ERROR: overly complex generic constant
   |  _____________^
LL | |         let x: Option<S> = None;
LL | |         //[min]~^ ERROR: generic parameters may not be used in const operations
LL | |         0
LL | |     }],
   | |_____^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to previous error
------------------------------------------

---
-    | |_____^ blocks are not supported in generic constant
+    | |_____^ blocks are not supported in generic
+             constant
12    |
13    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-2.full/issue-67945-2.full.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-67945-2.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67945-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-2.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-2.full/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL |       A: [(); {
   |  _____________^
LL | |         //[full]~^ ERROR overly complex generic constant
LL | |         let x: Option<Box<Self>> = None;
LL | |         //[min]~^ ERROR generic `Self` types are currently not permitted in anonymous constants
LL | |         0
LL | |     }],
   | |_____^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to previous error
------------------------------------------

---
-    | |_____^ blocks are not supported in generic constant
+    | |_____^ blocks are not supported in generic
+             constant
11    |
12    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-4.full/issue-67945-4.full.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-67945-4.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67945-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-4.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-4.full/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL |       A: [(); { //[full]~ ERROR: overly complex generic constant
   |  _____________^
LL | |         let x: Option<Box<S>> = None;
LL | |         //[min]~^ ERROR: generic parameters may not be used in const operations
LL | |         0
LL | |     }],
   | |_____^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-73260.rs stdout ----
diff of stderr:

+ error: unconstrained generic constant
+   --> $DIR/issue-73260.rs:16:32
+    |
+ LL |     let x: Arr<{usize::MAX}> = Arr {};
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); {N < usize::MAX / 2}]:`
+ note: required by a bound in `Arr`
+    |
+    |
+ LL | struct Arr<const N: usize>
+    |        --- required by a bound in this
+ LL | where
+ LL |     Assert::<{N < usize::MAX / 2}>: IsTrue,
+    |              ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Arr`
+ error: unconstrained generic constant
+   --> $DIR/issue-73260.rs:16:12
+    |
+    |
+ LL |     let x: Arr<{usize::MAX}> = Arr {};
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); {N < usize::MAX / 2}]:`
+ note: required by a bound in `Arr`
+    |
+    |
+ LL | struct Arr<const N: usize>
+    |        --- required by a bound in this
+ LL | where
+ LL |     Assert::<{N < usize::MAX / 2}>: IsTrue,
+    |              ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Arr`
1 error[E0308]: mismatched types
2   --> $DIR/issue-73260.rs:16:12
3    |

---
To only update this specific test, also pass `--test-args const-generics/issues/issue-73260.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-73260.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-73260" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-73260/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); {N < usize::MAX / 2}]:`
note: required by a bound in `Arr`
   |
   |
LL | struct Arr<const N: usize>
   |        --- required by a bound in this
LL | where
LL |     Assert::<{N < usize::MAX / 2}>: IsTrue,
   |              ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Arr`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-73260.rs:16:12
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); {N < usize::MAX / 2}]:`
note: required by a bound in `Arr`
   |
   |
LL | struct Arr<const N: usize>
   |        --- required by a bound in this
LL | where
LL |     Assert::<{N < usize::MAX / 2}>: IsTrue,
   |              ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Arr`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-73260.rs:16:12
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |            ^^^^^^^^^^^^^^^^^ expected `false`, found `true`
   = note: expected type `false`
              found type `true`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-73260.rs:16:32
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |                                ^^^ expected `false`, found `true`
   = note: expected type `false`
              found type `true`

error: aborting due to 4 previous errors
---
diff of stderr:

2   --> $DIR/issue-77357.rs:6:46
3    |
4 LL | fn bug<'a, T>() -> &'static dyn MyTrait<[(); { |x: &'a u32| { x }; 4 }]> {
-    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
6    |
6    |
7    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357/issue-77357.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357/issue-77357.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-77357.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-77357.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL | fn bug<'a, T>() -> &'static dyn MyTrait<[(); { |x: &'a u32| { x }; 4 }]> {
   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to previous error
------------------------------------------

---
- error[E0308]: mismatched types
+ error: unconstrained generic constant
2   --> $DIR/issue-79674.rs:26:5
3    |
4 LL |     requires_distinct("str", 12);
-    |     ^^^^^^^^^^^^^^^^^ expected `true`, found `false`
+    |     ^^^^^^^^^^^^^^^^^
6    |
-    = note: expected type `true`
-    = note: expected type `true`
-               found type `false`
+    = help: try adding a `where` bound using this expression: `where [(); {is_same_type::<A, B>()}]:`
+ note: required by a bound in `requires_distinct`
+    |
+    |
+ LL | fn requires_distinct<A, B>(_a: A, _b: B) where
+    |    ----------------- required by a bound in this
+ LL |     A: MiniTypeId, B: MiniTypeId,
+ LL |     Lift<{is_same_type::<A, B>()}>: IsFalse {}
+    |          ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `requires_distinct`
10 error: aborting due to previous error
11 

- For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-79674.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-79674.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-79674" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-79674/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     requires_distinct("str", 12);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); {is_same_type::<A, B>()}]:`
note: required by a bound in `requires_distinct`
   |
   |
LL | fn requires_distinct<A, B>(_a: A, _b: B) where
   |    ----------------- required by a bound in this
LL |     A: MiniTypeId, B: MiniTypeId,
LL |     Lift<{is_same_type::<A, B>()}>: IsFalse {}
   |          ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `requires_distinct`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-86033.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-86033.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86033" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86033/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL | fn _func() -> impl IsZST {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { std::mem::size_of::<T>() == 0 }]:`
note: required because of the requirements on the impl of `IsZST` for `[closure@/checkout/src/test/ui/const-generics/issues/issue-86033.rs:17:5: 17:7]`
   |
   |
LL | impl<T> IsZST for T

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-83249.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83249.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83249" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83249/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:17:17
   |
   |
LL |     let _: u8 = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:17:17
   |
LL |     let _: u8 = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:17:21
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:17:21
   |
LL |     let _: u8 = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:17:21
   |
LL |     let _: u8 = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:19:13
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:19:13
   |
LL |     let _ = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:19:13
   |
LL |     let _ = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:19:17
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:19:17
   |
LL |     let _ = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:19:17
   |
LL |     let _ = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
---
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

error: internal compiler error: encountered ambiguity selecting `Binder(<[type error] as Foo>, [])` during codegen, presuming due to overflow or prior type error
   |
LL |     const N: usize;
   |     ^^^^^^^^^^^^^^
   |
   |
   = note: delayed at compiler/rustc_ty_utils/src/instance.rs:213:37

error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:19:17
   |
LL |     let _ = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:749:18

error: internal compiler error: PromoteTemps: MIR had errors
   |
LL | pub fn bar() {
   | ^^^^^^^^^^^^
   |
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22

error: internal compiler error: broken MIR in DefId(0:10 ~ issue_83249[14ce]::bar) ("return type"): bad type [type error]
   |
LL | pub fn bar() {
   | ^^^^^^^^^^^^
   |
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:10 ~ issue_83249[14ce]::bar) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-83249.rs:16:1: 16:13 (#0), scope: scope[0] } }): bad type [type error]
   |
LL | pub fn bar() {
   | ^^^^^^^^^^^^
   |
---
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
   0:     0x7fb49ebfff9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0c98cd01c93ebbe0
   1:     0x7fb49ec669f8 - core::fmt::write::h606db46652223230
   2:     0x7fb49ebf05b1 - std::io::Write::write_fmt::h913f25516b02c551
   3:     0x7fb49ec02f5e - std::panicking::default_hook::{{closure}}::h37f68a43034fd25c
   4:     0x7fb49ec02c1f - std::panicking::default_hook::hdedf3e157f33a313
   5:     0x7fb49f5b9034 - rustc_driver[70c925622ea52f52]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb49ec03712 - std::panicking::rust_panic_with_hook::h1e23127ce09aec8d
   7:     0x7fb4a2363453 - std[927deef5ab2b67cf]::panicking::begin_panic::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>::{closure#0}
   8:     0x7fb4a2363016 - std[927deef5ab2b67cf]::sys_common::backtrace::__rust_end_short_backtrace::<std[927deef5ab2b67cf]::panicking::begin_panic<rustc_errors[9be4dd1e4e867516]::ExplicitBug>::{closure#0}, !>
   9:     0x7fb49f57a3b6 - std[927deef5ab2b67cf]::panicking::begin_panic::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>
  10:     0x7fb4a23a7306 - std[927deef5ab2b67cf]::panic::panic_any::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>
  11:     0x7fb4a23ac07c - <rustc_errors[9be4dd1e4e867516]::HandlerInner as core[ca24767fef18cf3e]::ops::drop::Drop>::drop
  12:     0x7fb49f5d8f32 - core[ca24767fef18cf3e]::ptr::drop_in_place::<rustc_session[dd587f99c778501]::parse::ParseSess>
  13:     0x7fb49f5e06b8 - <alloc[4c1670aac7f07936]::rc::Rc<rustc_session[dd587f99c778501]::session::Session> as core[ca24767fef18cf3e]::ops::drop::Drop>::drop
  14:     0x7fb49f5ac82c - core[ca24767fef18cf3e]::ptr::drop_in_place::<rustc_interface[83f48bbafc7ec5c9]::interface::Compiler>
  15:     0x7fb49f5aae1d - rustc_span[7eedac3fedda45e5]::with_source_map::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_interface[83f48bbafc7ec5c9]::interface::create_compiler_and_run<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fb49f5bd72a - rustc_interface[83f48bbafc7ec5c9]::interface::create_compiler_and_run::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>
  17:     0x7fb49f5a4b92 - <scoped_tls[285505d4febb8c5a]::ScopedKey<rustc_span[7eedac3fedda45e5]::SessionGlobals>>::set::<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>
  18:     0x7fb49f5dc89f - std[927deef5ab2b67cf]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>
  19:     0x7fb49f6133ae - std[927deef5ab2b67cf]::panicking::try::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, core[ca24767fef18cf3e]::panic::unwind_safe::AssertUnwindSafe<<std[927deef5ab2b67cf]::thread::Builder>::spawn_unchecked_<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7fb49f5de3d2 - <<std[927deef5ab2b67cf]::thread::Builder>::spawn_unchecked_<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#1} as core[ca24767fef18cf3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7fb49ec0e9e5 - std::sys::unix::thread::Thread::new::thread_start::h78e4691156550d1e
  22:     0x7fb499159609 - start_thread
  23:     0x7fb49ea6c133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (1f90b9c76 2022-08-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-84659.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-84659.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-84659" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-84659/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/const-generics/issues/issue-84659.rs:8:15
   |
   |
LL |     type Baz: Bar<{ Self::N }>;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-84659.rs:8:15
   |
LL |     type Baz: Bar<{ Self::N }>;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
stack backtrace:
   0:     0x7f216f45af9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0c98cd01c93ebbe0
   1:     0x7f216f4c19f8 - core::fmt::write::h606db46652223230
   2:     0x7f216f44b5b1 - std::io::Write::write_fmt::h913f25516b02c551
   3:     0x7f216f45df5e - std::panicking::default_hook::{{closure}}::h37f68a43034fd25c
   4:     0x7f216f45dc1f - std::panicking::default_hook::hdedf3e157f33a313
   5:     0x7f216fe14034 - rustc_driver[70c925622ea52f52]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f216f45e712 - std::panicking::rust_panic_with_hook::h1e23127ce09aec8d
   7:     0x7f2172bbe453 - std[927deef5ab2b67cf]::panicking::begin_panic::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>::{closure#0}
   8:     0x7f2172bbe016 - std[927deef5ab2b67cf]::sys_common::backtrace::__rust_end_short_backtrace::<std[927deef5ab2b67cf]::panicking::begin_panic<rustc_errors[9be4dd1e4e867516]::ExplicitBug>::{closure#0}, !>
   9:     0x7f216fdd53b6 - std[927deef5ab2b67cf]::panicking::begin_panic::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>
  10:     0x7f2172c02306 - std[927deef5ab2b67cf]::panic::panic_any::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>
  11:     0x7f2172c0707c - <rustc_errors[9be4dd1e4e867516]::HandlerInner as core[ca24767fef18cf3e]::ops::drop::Drop>::drop
  12:     0x7f216fe33f32 - core[ca24767fef18cf3e]::ptr::drop_in_place::<rustc_session[dd587f99c778501]::parse::ParseSess>
  13:     0x7f216fe3b6b8 - <alloc[4c1670aac7f07936]::rc::Rc<rustc_session[dd587f99c778501]::session::Session> as core[ca24767fef18cf3e]::ops::drop::Drop>::drop
  14:     0x7f216fe0782c - core[ca24767fef18cf3e]::ptr::drop_in_place::<rustc_interface[83f48bbafc7ec5c9]::interface::Compiler>
  15:     0x7f216fe05e1d - rustc_span[7eedac3fedda45e5]::with_source_map::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_interface[83f48bbafc7ec5c9]::interface::create_compiler_and_run<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f216fe1872a - rustc_interface[83f48bbafc7ec5c9]::interface::create_compiler_and_run::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>
  17:     0x7f216fdffb92 - <scoped_tls[285505d4febb8c5a]::ScopedKey<rustc_span[7eedac3fedda45e5]::SessionGlobals>>::set::<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>
  18:     0x7f216fe3789f - std[927deef5ab2b67cf]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>
  19:     0x7f216fe6e3ae - std[927deef5ab2b67cf]::panicking::try::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, core[ca24767fef18cf3e]::panic::unwind_safe::AssertUnwindSafe<<std[927deef5ab2b67cf]::thread::Builder>::spawn_unchecked_<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f216fe393d2 - <<std[927deef5ab2b67cf]::thread::Builder>::spawn_unchecked_<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#1} as core[ca24767fef18cf3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f216f4699e5 - std::sys::unix::thread::Thread::new::thread_start::h78e4691156550d1e
  22:     0x7f21699b4609 - start_thread
  23:     0x7f216f2c7133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (1f90b9c76 2022-08-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-83288.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83288.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83288" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83288/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:63:19
   |
   |
LL |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `I::NUM_ELEMS`, found `<I as Concat<J>>::Output::NUM_ELEMS`
   |
   = note: expected type `I::NUM_ELEMS`
              found type `<I as Concat<J>>::Output::NUM_ELEMS`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/issues/issue-88119.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-88119.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-88119" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-88119/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL | pub const ICE_1: &'static [u8] = <&&mut u8 as ConstName>::NAME_BYTES;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); name_len::<T>()]:`
note: required because of the requirements on the impl of `ConstName` for `&&mut u8`
   |
   |
LL | impl<T: ?Sized + ConstName> const ConstName for &T

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-88119.rs:32:34
   |
   |
LL | pub const ICE_1: &'static [u8] = <&&mut u8 as ConstName>::NAME_BYTES;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); name_len::<T>()]:`
note: required because of the requirements on the impl of `ConstName` for `&mut u8`
   |
   |
LL | impl<T: ?Sized + ConstName> const ConstName for &mut T
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `ConstName` for `&&mut u8`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-88119.rs:33:34
   |
   |
LL | pub const ICE_2: &'static [u8] = <&mut &u8 as ConstName>::NAME_BYTES;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); name_len::<T>()]:`
note: required because of the requirements on the impl of `ConstName` for `&mut &u8`
   |
   |
LL | impl<T: ?Sized + ConstName> const ConstName for &mut T

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-88119.rs:33:34
   |
   |
LL | pub const ICE_2: &'static [u8] = <&mut &u8 as ConstName>::NAME_BYTES;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); name_len::<T>()]:`
note: required because of the requirements on the impl of `ConstName` for `&u8`
   |
   |
LL | impl<T: ?Sized + ConstName> const ConstName for &T
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `ConstName` for `&mut &u8`

error: aborting due to 4 previous errors
error: aborting due to 4 previous errors
------------------------------------------


---- [ui] src/test/ui/const-generics/issues/issue-83765.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when resolving instance `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`
-   --> $DIR/issue-83765.rs:5:5
+ error[E0308]: method not compatible with trait
3    |
3    |
- LL |     const DIM: usize;
-    |     ^^^^^^^^^^^^^^^^
+ LL |     fn size(&self) -> [usize; DIM] {
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
6    |
- note: ...which requires checking if `TensorDimension` fulfills its obligations...
-   --> $DIR/issue-83765.rs:4:1
+    = note: expected type `Self::DIM`
+               found type `DIM`
+ error[E0308]: method not compatible with trait
+   --> $DIR/issue-83765.rs:57:5
9    |
9    |
- LL | trait TensorDimension {
-    | ^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which again requires resolving instance `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`, completing the cycle
- note: cycle used when checking if `TensorDimension` fulfills its obligations
-   --> $DIR/issue-83765.rs:4:1
+ LL |     fn bget(&self, index: [usize; DIM]) -> Option<Self::Element> {
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
15    |
- LL | trait TensorDimension {
+    = note: expected type `Self::DIM`
+               found type `DIM`
18 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0308]: method not compatible with trait
+   --> $DIR/issue-83765.rs:81:5
+    |
+ LL |     fn size(&self) -> [usize; DIM] {
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
+    = note: expected type `Self::DIM`
+               found type `DIM`
20 
- For more information about this error, try `rustc --explain E0391`.
- For more information about this error, try `rustc --explain E0391`.
+ error[E0308]: method not compatible with trait
+   --> $DIR/issue-83765.rs:90:5
+    |
+ LL |     fn bget(&self, index: [usize; DIM]) -> Option<Self::Element> {
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
+    = note: expected type `Self::DIM`
+               found type `DIM`
+ 
+ error[E0308]: mismatched types
+ error[E0308]: mismatched types
+   --> $DIR/issue-83765.rs:59:27
+    |
+ LL |         if !self.inbounds(index) {
+    |                           ^^^^^ expected `Self::DIM`, found `DIM`
+    = note: expected type `Self::DIM`
+               found type `DIM`
+ 
+ 
+ error[E0277]: the trait bound `[usize; _]: Default` is not satisfied
+    |
+    |
+ LL |         let newindex: [usize; T::DIM] = Default::default();
+    |                                         ^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `[usize; _]`
+    |
+ help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> where [usize; _]: Default {
+ 
+ error[E0308]: mismatched types
+   --> $DIR/issue-83765.rs:64:29
+    |
+    |
+ LL |         self.reference.bget(newindex)
+    |                             ^^^^^^^^ expected `Self::DIM`, found `T::DIM`
+    = note: expected type `Self::DIM`
+    = note: expected type `Self::DIM`
+               found type `T::DIM`
+ error[E0308]: mismatched types
+   --> $DIR/issue-83765.rs:82:9
+    |
+ LL |         self.reference.size()
+ LL |         self.reference.size()
+    |         ^^^^^^^^^^^^^^^^^^^^^ expected `DIM`, found `Self::DIM`
+    = note: expected type `DIM`
+               found type `Self::DIM`
+ 
+ error[E0308]: mismatched types
+ error[E0308]: mismatched types
+   --> $DIR/issue-83765.rs:91:29
+    |
+ LL |         self.reference.bget(index).map(&self.closure)
+    |                             ^^^^^ expected `Self::DIM`, found `DIM`
+    = note: expected type `Self::DIM`
+               found type `DIM`
+ 
+ 
+ error[E0599]: the method `bmap` exists for struct `LazyUpdim<'_, Vec<{integer}>, { Self::DIM }, 2>`, but its trait bounds were not satisfied
+    |
+    |
+ LL | struct LazyUpdim<'a, T: Broadcastable, const OLDDIM: usize, const DIM: usize> {
+    | |
+    | |
+    | method `bmap` not found for this struct
+    | doesn't satisfy `_: Broadcastable`
+ ...
+ LL |     let bbv = bv.bmap(|x| x * x);
+    |                  ^^^^ method cannot be called on `LazyUpdim<'_, Vec<{integer}>, { Self::DIM }, 2>` due to unsatisfied trait bounds
+ note: the following trait bounds were not satisfied:
+   --> $DIR/issue-83765.rs:55:81
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |                                                                                 |
+    |                                                                                 unsatisfied trait bound introduced here
+ 
+ error: aborting due to 10 previous errors
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-83765.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83765.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: method not compatible with trait
   |
   |
LL |     fn size(&self) -> [usize; DIM] {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: method not compatible with trait
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:57:5
   |
LL |     fn bget(&self, index: [usize; DIM]) -> Option<Self::Element> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: method not compatible with trait
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:81:5
   |
LL |     fn size(&self) -> [usize; DIM] {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: method not compatible with trait
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:90:5
   |
LL |     fn bget(&self, index: [usize; DIM]) -> Option<Self::Element> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:59:27
   |
LL |         if !self.inbounds(index) {
   |                           ^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`


error[E0277]: the trait bound `[usize; _]: Default` is not satisfied
   |
   |
LL |         let newindex: [usize; T::DIM] = Default::default();
   |                                         ^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `[usize; _]`
   |
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> where [usize; _]: Default {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:64:29
   |
   |
LL |         self.reference.bget(newindex)
   |                             ^^^^^^^^ expected `Self::DIM`, found `T::DIM`
   = note: expected type `Self::DIM`
   = note: expected type `Self::DIM`
              found type `T::DIM`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:82:9
   |
LL |         self.reference.size()
LL |         self.reference.size()
   |         ^^^^^^^^^^^^^^^^^^^^^ expected `DIM`, found `Self::DIM`
   = note: expected type `DIM`
              found type `Self::DIM`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:91:29
   |
LL |         self.reference.bget(index).map(&self.closure)
   |                             ^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`


error[E0599]: the method `bmap` exists for struct `LazyUpdim<'_, Vec<{integer}>, { Self::DIM }, 2>`, but its trait bounds were not satisfied
   |
   |
LL | struct LazyUpdim<'a, T: Broadcastable, const OLDDIM: usize, const DIM: usize> {
   | |
   | |
   | method `bmap` not found for this struct
   | doesn't satisfy `_: Broadcastable`
...
LL |     let bbv = bv.bmap(|x| x * x);
   |                  ^^^^ method cannot be called on `LazyUpdim<'_, Vec<{integer}>, { Self::DIM }, 2>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:55:81
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |                                                                                 |
   |                                                                                 unsatisfied trait bound introduced here

error: aborting due to 10 previous errors
---
---- [ui] src/test/ui/const-generics/issues/issue-89304.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-89304.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89304" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89304/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0283]: type annotations needed: cannot satisfy `GenericStruct<{T + 1}>: From<GenericStruct<T>>`
   |
   |
LL | impl<const T: usize> From<GenericStruct<T>> for GenericStruct<{T + 1}> {
   |
   |
note: multiple `impl`s satisfying `GenericStruct<{T + 1}>: From<GenericStruct<T>>` found
   |
   |
LL | impl<const T: usize> From<GenericStruct<T>> for GenericStruct<{T + 1}> {
...
...
LL | impl<const T: usize> From<GenericStruct<{T + 1}>> for GenericStruct<T> {


error[E0283]: type annotations needed: cannot satisfy `GenericStruct<T>: From<GenericStruct<{T + 1}>>`
   |
   |
LL | impl<const T: usize> From<GenericStruct<{T + 1}>> for GenericStruct<T> {
   |
   |
note: multiple `impl`s satisfying `GenericStruct<T>: From<GenericStruct<{T + 1}>>` found
   |
   |
LL | impl<const T: usize> From<GenericStruct<T>> for GenericStruct<{T + 1}> {
...
...
LL | impl<const T: usize> From<GenericStruct<{T + 1}>> for GenericStruct<T> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0283`.
For more information about this error, try `rustc --explain E0283`.
------------------------------------------


---- [ui] src/test/ui/const-generics/issues/issue-89146.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-89146.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89146" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89146/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:23:7
   |
   |
LL |     a.to_bytes()[0]
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:23:7
   |
LL |     a.to_bytes()[0]
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:23:5
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:23:5
   |
LL |     a.to_bytes()[0]
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:23:5
   |
LL |     a.to_bytes()[0]
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:749:18

error: internal compiler error: PromoteTemps: MIR had errors
   |
   |
LL | / fn deeper_bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22


error: internal compiler error: broken MIR in DefId(0:10 ~ issue_89146[a52a]::deeper_bar) ("return type"): bad type [type error]
   |
   |
LL | / fn deeper_bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:10 ~ issue_89146[a52a]::deeper_bar) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-89146.rs:19:1: 21:26 (#0), scope: scope[0] } }): bad type [type error]
   |
   |
LL | / fn deeper_bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:10 ~ issue_89146[a52a]::deeper_bar) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-89146.rs:19:1: 21:26 (#0), scope: scope[0] } }): bad type [type error]
   |
   |
LL | / fn deeper_bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
---
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
   0:     0x7f800b9b8f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0c98cd01c93ebbe0
   1:     0x7f800ba1f9f8 - core::fmt::write::h606db46652223230
   2:     0x7f800b9a95b1 - std::io::Write::write_fmt::h913f25516b02c551
   3:     0x7f800b9bbf5e - std::panicking::default_hook::{{closure}}::h37f68a43034fd25c
   4:     0x7f800b9bbc1f - std::panicking::default_hook::hdedf3e157f33a313
   5:     0x7f800c372034 - rustc_driver[70c925622ea52f52]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f800b9bc712 - std::panicking::rust_panic_with_hook::h1e23127ce09aec8d
   7:     0x7f800f11c453 - std[927deef5ab2b67cf]::panicking::begin_panic::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>::{closure#0}
   8:     0x7f800f11c016 - std[927deef5ab2b67cf]::sys_common::backtrace::__rust_end_short_backtrace::<std[927deef5ab2b67cf]::panicking::begin_panic<rustc_errors[9be4dd1e4e867516]::ExplicitBug>::{closure#0}, !>
   9:     0x7f800c3333b6 - std[927deef5ab2b67cf]::panicking::begin_panic::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>
  10:     0x7f800f160306 - std[927deef5ab2b67cf]::panic::panic_any::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>
  11:     0x7f800f16507c - <rustc_errors[9be4dd1e4e867516]::HandlerInner as core[ca24767fef18cf3e]::ops::drop::Drop>::drop
  12:     0x7f800c391f32 - core[ca24767fef18cf3e]::ptr::drop_in_place::<rustc_session[dd587f99c778501]::parse::ParseSess>
  13:     0x7f800c3996b8 - <alloc[4c1670aac7f07936]::rc::Rc<rustc_session[dd587f99c778501]::session::Session> as core[ca24767fef18cf3e]::ops::drop::Drop>::drop
  14:     0x7f800c36582c - core[ca24767fef18cf3e]::ptr::drop_in_place::<rustc_interface[83f48bbafc7ec5c9]::interface::Compiler>
  15:     0x7f800c363e1d - rustc_span[7eedac3fedda45e5]::with_source_map::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_interface[83f48bbafc7ec5c9]::interface::create_compiler_and_run<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f800c37672a - rustc_interface[83f48bbafc7ec5c9]::interface::create_compiler_and_run::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>
  17:     0x7f800c35db92 - <scoped_tls[285505d4febb8c5a]::ScopedKey<rustc_span[7eedac3fedda45e5]::SessionGlobals>>::set::<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>
  18:     0x7f800c39589f - std[927deef5ab2b67cf]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>
  19:     0x7f800c3cc3ae - std[927deef5ab2b67cf]::panicking::try::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, core[ca24767fef18cf3e]::panic::unwind_safe::AssertUnwindSafe<<std[927deef5ab2b67cf]::thread::Builder>::spawn_unchecked_<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f800c3973d2 - <<std[927deef5ab2b67cf]::thread::Builder>::spawn_unchecked_<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#1} as core[ca24767fef18cf3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f800b9c79e5 - std::sys::unix::thread::Thread::new::thread_start::h78e4691156550d1e
  22:     0x7f8005f12609 - start_thread
  23:     0x7f800b825133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (1f90b9c76 2022-08-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/occurs-check/unify-fixpoint.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/occurs-check/unify-fixpoint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unify-fixpoint" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unify-fixpoint/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(generic_const_exprs)] //~ WARN the feature `generic_const_exprs` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information


error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/occurs-check/unify-fixpoint.rs:11:11
   |
LL |     arr = bind::<2>(arr);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 2]:`
note: required by a bound in `bind`
   |
   |
LL | fn bind<const N: usize>(value: [u8; N + 2]) -> [u8; N * 2] {
   |                                     ^^^^^ required by this bound in `bind`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/occurs-check/unify-fixpoint.rs:11:11
   |
   |
LL |     arr = bind::<2>(arr);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N * 2]:`
note: required by a bound in `bind`
   |
   |
LL | fn bind<const N: usize>(value: [u8; N + 2]) -> [u8; N * 2] {
   |                                                     ^^^^^ required by this bound in `bind`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/occurs-check/unify-fixpoint.rs:10:19
   |
LL |     let mut arr = Default::default();
LL |     let mut arr = Default::default();
   |                   ^^^^^^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 2]:`
note: required by a bound in `bind`
   |
   |
LL | fn bind<const N: usize>(value: [u8; N + 2]) -> [u8; N * 2] {
   |                                     ^^^^^ required by this bound in `bind`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/occurs-check/unify-fixpoint.rs:10:19
   |
LL |     let mut arr = Default::default();
LL |     let mut arr = Default::default();
   |                   ^^^^^^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); N * 2]:`
note: required by a bound in `bind`
   |
   |
LL | fn bind<const N: usize>(value: [u8; N + 2]) -> [u8; N * 2] {
   |                                                     ^^^^^ required by this bound in `bind`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/occurs-check/unify-fixpoint.rs:11:11
   |
   |
LL |     arr = bind::<2>(arr);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 2]:`
note: required by a bound in `bind`
   |
   |
LL | fn bind<const N: usize>(value: [u8; N + 2]) -> [u8; N * 2] {
   |                                     ^^^^^ required by this bound in `bind`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/occurs-check/unify-fixpoint.rs:11:11
   |
   |
LL |     arr = bind::<2>(arr);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N * 2]:`
note: required by a bound in `bind`
   |
   |
LL | fn bind<const N: usize>(value: [u8; N + 2]) -> [u8; N * 2] {
   |                                                     ^^^^^ required by this bound in `bind`
error: aborting due to 6 previous errors; 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/const-generics/occurs-check/unify-n-nplusone.rs stdout ----
diff of stderr:

+ error: unconstrained generic constant
+   --> $DIR/unify-n-nplusone.rs:14:11
+    |
+ LL |     arr = bind(arr);
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
+ note: required by a bound in `bind`
+    |
+    |
+ LL | fn bind<const N: usize>(value: [u8; N]) -> [u8; N + 1] {
+    |                                                 ^^^^^ required by this bound in `bind`
+ error: unconstrained generic constant
+   --> $DIR/unify-n-nplusone.rs:14:11
+    |
+    |
+ LL |     arr = bind(arr);
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
+ note: required by a bound in `bind`
+    |
+    |
+ LL | fn bind<const N: usize>(value: [u8; N]) -> [u8; N + 1] {
+    |                                                 ^^^^^ required by this bound in `bind`
1 error[E0308]: mismatched types
2   --> $DIR/unify-n-nplusone.rs:14:11
3    |


4 LL |     arr = bind(arr);
6 
- error: aborting due to previous error
+ error: aborting due to 3 previous errors
8 
---
To only update this specific test, also pass `--test-args const-generics/occurs-check/unify-n-nplusone.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/occurs-check/unify-n-nplusone.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unify-n-nplusone" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unify-n-nplusone/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     arr = bind(arr); //~ ERROR mismatched types
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `bind`
   |
   |
LL | fn bind<const N: usize>(value: [u8; N]) -> [u8; N + 1] {
   |                                                 ^^^^^ required by this bound in `bind`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/occurs-check/unify-n-nplusone.rs:14:11
   |
   |
LL |     arr = bind(arr); //~ ERROR mismatched types
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `bind`
   |
   |
LL | fn bind<const N: usize>(value: [u8; N]) -> [u8; N + 1] {
   |                                                 ^^^^^ required by this bound in `bind`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/occurs-check/unify-n-nplusone.rs:14:11
   |
   |
LL |     arr = bind(arr); //~ ERROR mismatched types

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
---
- error[E0284]: type annotations needed
+ error: unconstrained generic constant
2   --> $DIR/parent_generics_of_encoding_impl_trait.rs:9:5
3    |
4 LL |     generics_of_parent_impl_trait::foo([()]);

-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer the value of const parameter `N` declared on the function `foo`
6    |
6    |
+    = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
7 note: required by a bound in `foo`
8   --> $DIR/auxiliary/generics_of_parent_impl_trait.rs:6:48

12 
13 error: aborting due to previous error
14 
---
To only update this specific test, also pass `--test-args const-generics/parent_generics_of_encoding_impl_trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/parent_generics_of_encoding_impl_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/parent_generics_of_encoding_impl_trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/parent_generics_of_encoding_impl_trait/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     generics_of_parent_impl_trait::foo([()]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `foo`
  --> /checkout/src/test/ui/const-generics/auxiliary/generics_of_parent_impl_trait.rs:6:48
   |
LL | pub fn foo<const N: usize>(foo: impl Into<[(); N + 1]>) {
   |                                                ^^^^^ required by this bound in `foo`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/parent_generics_of_encoding.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/parent_generics_of_encoding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/parent_generics_of_encoding" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/parent_generics_of_encoding/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let x: S<u8, N> = S::test();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N * 2]:`
note: required by a bound in `S`
  --> /checkout/src/test/ui/const-generics/auxiliary/generics_of_parent.rs:12:9
   |
LL |     [T; N * 2]: Sized,
   |         ^^^^^ required by this bound in `S`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/parent_generics_of_encoding.rs:13:23
   |
   |
LL |     let x: S<u8, N> = S::test();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N * 2]:`
note: required by a bound in `S`
  --> /checkout/src/test/ui/const-generics/auxiliary/generics_of_parent.rs:12:9
   |
LL |     [T; N * 2]: Sized,
   |         ^^^^^ required by this bound in `S`

error[E0599]: no function or associated item named `test` found for struct `S<_, _>` in the current scope
   |
   |
LL |     let x: S<u8, N> = S::test();
   |                          ^^^^ function or associated item cannot be called on `S<_, _>` due to unsatisfied trait bounds
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/try_unify_ignore_lifetimes.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/try_unify_ignore_lifetimes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/try_unify_ignore_lifetimes/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs:20:21
   |
   |
LL |     fn bar<'a>(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs:20:21
   |
LL |     fn bar<'a>(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs:28:17
  --> /checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs:28:17
   |
LL |     fn baz(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs:28:17
   |
LL |     fn baz(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
stack backtrace:
   0:     0x7f2ef2283f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0c98cd01c93ebbe0
   1:     0x7f2ef22ea9f8 - core::fmt::write::h606db46652223230
   2:     0x7f2ef22745b1 - std::io::Write::write_fmt::h913f25516b02c551
   3:     0x7f2ef2286f5e - std::panicking::default_hook::{{closure}}::h37f68a43034fd25c
   4:     0x7f2ef2286c1f - std::panicking::default_hook::hdedf3e157f33a313
   5:     0x7f2ef2c3d034 - rustc_driver[70c925622ea52f52]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2ef2287712 - std::panicking::rust_panic_with_hook::h1e23127ce09aec8d
   7:     0x7f2ef59e7453 - std[927deef5ab2b67cf]::panicking::begin_panic::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>::{closure#0}
   8:     0x7f2ef59e7016 - std[927deef5ab2b67cf]::sys_common::backtrace::__rust_end_short_backtrace::<std[927deef5ab2b67cf]::panicking::begin_panic<rustc_errors[9be4dd1e4e867516]::ExplicitBug>::{closure#0}, !>
   9:     0x7f2ef2bfe3b6 - std[927deef5ab2b67cf]::panicking::begin_panic::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>
  10:     0x7f2ef5a2b306 - std[927deef5ab2b67cf]::panic::panic_any::<rustc_errors[9be4dd1e4e867516]::ExplicitBug>
  11:     0x7f2ef5a3007c - <rustc_errors[9be4dd1e4e867516]::HandlerInner as core[ca24767fef18cf3e]::ops::drop::Drop>::drop
  12:     0x7f2ef2c5cf32 - core[ca24767fef18cf3e]::ptr::drop_in_place::<rustc_session[dd587f99c778501]::parse::ParseSess>
  13:     0x7f2ef2c646b8 - <alloc[4c1670aac7f07936]::rc::Rc<rustc_session[dd587f99c778501]::session::Session> as core[ca24767fef18cf3e]::ops::drop::Drop>::drop
  14:     0x7f2ef2c3082c - core[ca24767fef18cf3e]::ptr::drop_in_place::<rustc_interface[83f48bbafc7ec5c9]::interface::Compiler>
  15:     0x7f2ef2c2ee1d - rustc_span[7eedac3fedda45e5]::with_source_map::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_interface[83f48bbafc7ec5c9]::interface::create_compiler_and_run<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f2ef2c4172a - rustc_interface[83f48bbafc7ec5c9]::interface::create_compiler_and_run::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>
  17:     0x7f2ef2c28b92 - <scoped_tls[285505d4febb8c5a]::ScopedKey<rustc_span[7eedac3fedda45e5]::SessionGlobals>>::set::<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>
  18:     0x7f2ef2c6089f - std[927deef5ab2b67cf]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>
  19:     0x7f2ef2c973ae - std[927deef5ab2b67cf]::panicking::try::<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, core[ca24767fef18cf3e]::panic::unwind_safe::AssertUnwindSafe<<std[927deef5ab2b67cf]::thread::Builder>::spawn_unchecked_<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f2ef2c623d2 - <<std[927deef5ab2b67cf]::thread::Builder>::spawn_unchecked_<rustc_interface[83f48bbafc7ec5c9]::util::run_in_thread_pool_with_globals<rustc_interface[83f48bbafc7ec5c9]::interface::run_compiler<core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>, rustc_driver[70c925622ea52f52]::run_compiler::{closure#1}>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#0}, core[ca24767fef18cf3e]::result::Result<(), rustc_errors[9be4dd1e4e867516]::ErrorGuaranteed>>::{closure#1} as core[ca24767fef18cf3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f2ef22929e5 - std::sys::unix::thread::Thread::new::thread_start::h78e4691156550d1e
  22:     0x7f2eec7dd609 - start_thread
  23:     0x7f2ef20f0133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (1f90b9c76 2022-08-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/occurs-check/unused-substs-5.rs stdout ----
diff of stderr:

+ error: unconstrained generic constant
+   --> $DIR/unused-substs-5.rs:15:9
+    |
+ LL |     x = q::<_, N>(x);
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
+ note: required by a bound in `q`
+   --> $DIR/unused-substs-5.rs:5:39
+    |
+ LL | fn q<T, const N: usize>(_: T) -> [u8; N + 1] {
+    |                                       ^^^^^ required by this bound in `q`
+ error: unconstrained generic constant
+   --> $DIR/unused-substs-5.rs:15:9
+    |
+    |
+ LL |     x = q::<_, N>(x);
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
+ note: required by a bound in `q`
+   --> $DIR/unused-substs-5.rs:5:39
+    |
+ LL | fn q<T, const N: usize>(_: T) -> [u8; N + 1] {
+    |                                       ^^^^^ required by this bound in `q`
1 error[E0308]: mismatched types
2   --> $DIR/unused-substs-5.rs:15:9
3    |


6    |         |
7    |         cyclic type of infinite size
- error: aborting due to previous error
+ error: unconstrained generic constant
+   --> $DIR/unused-substs-5.rs:19:5
+    |
+    |
+ LL |     catch_me::<3>();
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
+ note: required by a bound in `catch_me`
+   --> $DIR/unused-substs-5.rs:13:42
+    |
+ LL | fn catch_me<const N: usize>() where [u8; N + 1]: Default {
+    |                                          ^^^^^ required by this bound in `catch_me`
+ error: aborting due to 4 previous errors
10 
11 For more information about this error, try `rustc --explain E0308`.
12 
12 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-5/unused-substs-5.stderr
To only update this specific test, also pass `--test-args const-generics/occurs-check/unused-substs-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/occurs-check/unused-substs-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-5" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-5/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     x = q::<_, N>(x); //~ ERROR mismatched types
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `q`
   |
   |
LL | fn q<T, const N: usize>(_: T) -> [u8; N + 1] {
   |                                       ^^^^^ required by this bound in `q`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/occurs-check/unused-substs-5.rs:15:9
   |
   |
LL |     x = q::<_, N>(x); //~ ERROR mismatched types
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `q`
   |
   |
LL | fn q<T, const N: usize>(_: T) -> [u8; N + 1] {
   |                                       ^^^^^ required by this bound in `q`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/occurs-check/unused-substs-5.rs:15:9
   |
   |
LL |     x = q::<_, N>(x); //~ ERROR mismatched types
   |         ^^^^^^^^^^^^- help: try using a conversion method: `.to_vec()`
   |         |
   |         cyclic type of infinite size
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/occurs-check/unused-substs-5.rs:19:5
   |
   |
LL |     catch_me::<3>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N + 1]:`
note: required by a bound in `catch_me`
   |
   |
LL | fn catch_me<const N: usize>() where [u8; N + 1]: Default {
   |                                          ^^^^^ required by this bound in `catch_me`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---

52    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
53    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
54 
- error[E0446]: private type `fn(u8) -> u8 {my_const_fn}` in public interface
-   --> $DIR/where-priv-type.rs:80:5
-    |
- LL |     type AssocTy = Const<{ my_const_fn(U) }>;
-    |     ^^^^^^^^^^^^ can't leak private type
- ...
- LL | const fn my_const_fn(val: u8) -> u8 {
-    | ----------------------------------- `fn(u8) -> u8 {my_const_fn}` declared as private
- error: aborting due to 2 previous errors; 4 warnings emitted
+ error: aborting due to previous error; 4 warnings emitted
65 
66 For more information about this error, try `rustc --explain E0446`.
---
To only update this specific test, also pass `--test-args privacy/where-priv-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/where-priv-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/where-priv-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/where-priv-type/auxiliary"
stdout: none
--- stderr -------------------------------
warning: private type `PrivTy` in public interface (error E0446)
   |
LL | pub struct S
   | ^^^^^^^^^^^^
   |
   |
   = note: `#[warn(private_in_public)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

warning: private type `PrivTy` in public interface (error E0446)
   |
LL | pub enum E
   | ^^^^^^^^^^
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

warning: private type `PrivTy` in public interface (error E0446)
   |
LL | / pub fn f()
LL | / pub fn f()
LL | | //~^ WARNING private type `PrivTy` in public interface
LL | | //~| WARNING hard error
LL | | where
LL | |     PrivTy:
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error[E0446]: private type `PrivTy` in public interface
   |
LL | struct PrivTy;
LL | struct PrivTy;
   | ------------- `PrivTy` declared as private
LL | impl S
   | ^^^^^^ can't leak private type


warning: private type `PrivTy` in public interface (error E0446)
   |
LL | /     pub fn f()
LL | /     pub fn f()
LL | |     //~^ WARNING private type `PrivTy` in public interface
LL | |     //~| WARNING hard error
LL | |     where
LL | |         PrivTy:
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

---

---- [ui] src/test/ui/simd/array-trait.rs stdout ----
diff of stderr:

- error: unconstrained generic constant
-   --> $DIR/array-trait.rs:23:23
+ error[E0077]: SIMD vector element type should be a primitive scalar (integer/float/pointer) type
3    |
3    |
4 LL | pub struct T<S: Simd>([S::Lane; S::SIZE]);
-    |
-    |
-    = help: try adding a `where` bound using this expression: `where [(); S::SIZE]:`
8 
9 error: aborting due to previous error
10 

---
To only update this specific test, also pass `--test-args simd/array-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/array-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/array-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/array-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0077]: SIMD vector element type should be a primitive scalar (integer/float/pointer) type
   |
   |
LL | pub struct T<S: Simd>([S::Lane; S::SIZE]);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0077`.
