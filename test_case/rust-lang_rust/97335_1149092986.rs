plain
running 13044 tests
........................................................................................ 88/13044
.................................................................iiiiiiiiiiiiii......... 176/13044
......i.i.................i...i......................................................... 264/13044
.....................................................FF......F.......................... 352/13044
........................................................................................ 528/13044
........................................................................................ 616/13044
........................................................................................ 704/13044
......................................i................................................. 792/13044
---
---- [ui] src/test/ui/associated-type-bounds/fn-dyn-apit.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/fn-dyn-apit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/fn-dyn-apit/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/fn-dyn-apit/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `for<'a> <_ as fn_dyn_aux::Epsilon<'a>>::Zeta: fn_dyn_aux::Eta` is not satisfied
   |
   |
LL |     desugared_bound_region_forall2(beta)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a> fn_dyn_aux::Eta` is not implemented for `<_ as fn_dyn_aux::Epsilon<'a>>::Zeta`
   |
   = help: the trait `fn_dyn_aux::Eta` is implemented for `ZetaType`
note: required by a bound in `fn_dyn_aux::desugared_bound_region_forall2`
  --> /checkout/src/test/ui/associated-type-bounds/auxiliary/fn-dyn-aux.rs:145:46
   |
LL |     for<'a> <B::Gamma as Epsilon<'a>>::Zeta: Eta,
   |                                              ^^^ required by this bound in `fn_dyn_aux::desugared_bound_region_forall2`
error: aborting due to previous error

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/associated-type-bounds/rpit.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/rpit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/rpit/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/rpit/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `<impl Tr1 + std::marker::Copy as Tr1>::As1: std::marker::Copy` is not satisfied
   |
   |
LL | pub fn use_et1() { assert_copy(def_et1().mk()); }
   |                    ----------- ^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `<impl Tr1 + std::marker::Copy as Tr1>::As1`
   |                    required by a bound introduced by this call
   |
note: required by a bound in `assert_copy`
  --> /checkout/src/test/ui/associated-type-bounds/rpit.rs:10:19
  --> /checkout/src/test/ui/associated-type-bounds/rpit.rs:10:19
   |
LL | fn assert_copy<T: Copy>(x: T) { let _x = x; let _x = x; }
   |                   ^^^^ required by this bound in `assert_copy`

error[E0599]: no method named `clone` found for associated type `<impl Tr1 + Clone + Into<u8> + Add<u8> + Iterator as Tr1>::As1` in the current scope
   |
   |
LL |     let _0 = def_et3().mk().clone();
   |                             ^^^^^ method not found in `<impl Tr1 + Clone + Into<u8> + Add<u8> + Iterator as Tr1>::As1`

error[E0277]: the trait bound `for<'a> <impl Tr1 + Tr2<for<'a> 'a> as Tr1>::As1: Tr2<'a>` is not satisfied
   |
   |
LL | pub fn use_et4() { assert_forall_tr2(def_et4().mk()); }
   |                    ----------------- ^^^^^^^^^^^^^^ the trait `for<'a> Tr2<'a>` is not implemented for `<impl Tr1 + Tr2<for<'a> 'a> as Tr1>::As1`
   |                    required by a bound introduced by this call
   |
   |
   = help: the trait `Tr2<'a>` is implemented for `def_et4::A`
note: required by a bound in `assert_forall_tr2`
   |
   |
LL | fn assert_forall_tr2<T: for<'a> Tr2<'a>>(_: T) {}
   |                         ^^^^^^^^^^^^^^^ required by this bound in `assert_forall_tr2`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/associated-type-bounds/trait-alias-impl-trait.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/trait-alias-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/trait-alias-impl-trait/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/trait-alias-impl-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `<Et1 as Tr1>::As1: std::marker::Copy` is not satisfied
   |
   |
LL |     assert_copy(def_et1().mk());
   |     ----------- ^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `<Et1 as Tr1>::As1`
   |     required by a bound introduced by this call
   |
note: required by a bound in `assert_copy`
  --> /checkout/src/test/ui/associated-type-bounds/trait-alias-impl-trait.rs:16:19
  --> /checkout/src/test/ui/associated-type-bounds/trait-alias-impl-trait.rs:16:19
   |
LL | fn assert_copy<T: Copy>(x: T) {
   |                   ^^^^ required by this bound in `assert_copy`

error[E0599]: no method named `clone` found for associated type `<Et3 as Tr1>::As1` in the current scope
   |
   |
LL |     let _0 = def_et3().mk().clone();
   |                             ^^^^^ method not found in `<Et3 as Tr1>::As1`

error[E0277]: the trait bound `for<'a> <Et4 as Tr1>::As1: Tr2<'a>` is not satisfied
   |
   |
LL |     assert_forall_tr2(def_et4().mk());
   |     ----------------- ^^^^^^^^^^^^^^ the trait `for<'a> Tr2<'a>` is not implemented for `<Et4 as Tr1>::As1`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Tr2<'a>` is implemented for `def_et4::A`
note: required by a bound in `assert_forall_tr2`
   |
   |
LL | fn assert_forall_tr2<T: for<'a> Tr2<'a>>(_: T) {}
   |                         ^^^^^^^^^^^^^^^ required by this bound in `assert_forall_tr2`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
