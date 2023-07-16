plain
---- [ui] checkout/tests/ui/lint/unused/issue-104397.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/unused/issue-104397.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/issue-104397" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/issue-104397/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/tests/ui/lint/unused/issue-104397.rs:13:5
   |
   |
LL |     (for<'a> fn(Inv<'a>)): Trait,
   |     ^                   ^
note: the lint level is defined here
  --> /checkout/tests/ui/lint/unused/issue-104397.rs:4:9
   |
LL | #![deny(warnings)]
LL | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(unused_parens)]` implied by `#[deny(warnings)]`
help: remove these parentheses
   |
LL -     (for<'a> fn(Inv<'a>)): Trait,
LL +     for<'a> fn(Inv<'a>): Trait,

error: aborting due to previous error
------------------------------------------

