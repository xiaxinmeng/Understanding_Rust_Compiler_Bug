plain
failures:

---- [ui] src/test/ui/never_type/fallback-closure-ret.rs#fallback stdout ----

error in revision `fallback`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/fallback-closure-ret.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fallback" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/fallback-closure-ret.fallback" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/fallback-closure-ret.fallback/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `!: Bar` is not satisfied
   |
   |
LL |     foo(|| panic!());
   |     ^^^ the trait `Bar` is not implemented for `!`
   = help: the following other types implement trait `Bar`:
             ()
             u32
             u32
   = note: this error might have been caused by changes to Rust's type-inference algorithm (see issue #48950 <https://github.com/rust-lang/rust/issues/48950> for more information)
   = help: did you intend to use the type `()` here instead?
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
note: required by a bound in `foo`
   |
   |
LL | fn foo<R: Bar>(_: impl Fn() -> R) {}
   |           ^^^ required by this bound in `foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
