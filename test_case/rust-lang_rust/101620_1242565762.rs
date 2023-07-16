plain
........................................................................................ 3784/13508
........................................................................................ 3872/13508
....i..........i..........i............................................................. 3960/13508
........................................................................................ 4048/13508
.ii.F................................................................................... 4136/13508
........................................................................................ 4312/13508
........................................................................................ 4400/13508
........................................................................................ 4488/13508
........................................................................................ 4576/13508
---

---- [ui] src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs stdout ----
diff of stderr:

119 LL +         Foo::C => todo!()
121 
- error: aborting due to previous error; 10 warnings emitted
+ warning: unknown lint: `non_exhaustive_omitted_patterns`
+   --> $DIR/feature-gate-non_exhaustive_omitted_patterns_lint.rs:3:1
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unknown lint: `non_exhaustive_omitted_patterns`
   |
LL | #![deny(non_exhaustive_omitted_patterns)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
---
   = note: the `non_exhaustive_omitted_patterns` lint is unstable
   = note: see issue #89554 <https://github.com/rust-lang/rust/issues/89554> for more information
   = help: add `#![feature(non_exhaustive_omitted_patterns_lint)]` to the crate attributes to enable

error[E0004]: non-exhaustive patterns: `Foo::C` not covered
   |
LL |     match Foo::A {
LL |     match Foo::A {
   |           ^^^^^^ pattern `Foo::C` not covered
note: `Foo` defined here
  --> /checkout/src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs:12:15
   |
LL |     enum Foo {
LL |     enum Foo {
   |          ---
LL |         A, B, C,
   |               ^ not covered
   = note: the matched value is of type `Foo`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
LL ~         Foo::B => {}
LL +         Foo::C => todo!()

warning: unknown lint: `non_exhaustive_omitted_patterns`
  --> /checkout/src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs:3:1
   |
