plain
---- [ui] src/test/ui/type-alias-impl-trait/issue-96572-unconstrained.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-96572-unconstrained.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-96572-unconstrained" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-96572-unconstrained/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0004]: non-exhaustive patterns: `_` not covered
   |
LL |     match foo {
   |           ^^^ pattern `_` not covered
   |
   |
   = note: the matched value is of type `main::T`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
LL ~         Some((a, b)) => (),
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL ~         _ => todo!(),

error[E0004]: non-exhaustive patterns: `_` not covered
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-96572-unconstrained.rs:28:15
   |
   |
LL |         match foo {
   |               ^^^ pattern `_` not covered
   |
   = note: the matched value is of type `enum_upvar::T`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
LL ~             Some((a, b)) => (),
LL ~             _ => todo!(),


error[E0004]: non-exhaustive patterns: `Some(_)` not covered
   |
LL |         match bar {
LL |         match bar {
   |               ^^^ pattern `Some(_)` not covered
   |
note: `Option<only_pattern::U>` defined here
   |
LL | pub enum Option<T> {
   | ------------------
...
...
LL |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
   |     ^^^^ not covered
   = note: the matched value is of type `Option<only_pattern::U>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
LL ~             None => {}
LL ~             None => {}
LL +             Some(_) => todo!()


error[E0004]: non-exhaustive patterns: `Some(_)` not covered
   |
   |
LL |         match bar(!b) {
   |               ^^^^^^^ pattern `Some(_)` not covered
   |
note: `Option<impl Copy>` defined here
   |
LL | pub enum Option<T> {
   | ------------------
...
...
LL |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
   |     ^^^^ not covered
   = note: the matched value is of type `Option<impl Copy>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
LL ~             None => {}
LL ~             None => {}
LL +             Some(_) => todo!()

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0004`.
