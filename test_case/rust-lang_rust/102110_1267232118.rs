plain

36    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
37    = note: `#[warn(anonymous_parameters)]` on by default
38 
- error[E0718]: `fn` language item must be applied to a trait with 1 generic argument
+ error[E0718]: `r#fn` language item must be applied to a trait with 1 generic argument
41    |
41    |
42 LL | #[lang = "fn"]

The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-83471/issue-83471.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-83471/issue-83471.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lang-items/issue-83471.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-items/issue-83471.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-83471" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-83471/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0573]: expected type, found built-in attribute `export_name`
   |
LL |     fn call(export_name);
   |             ^^^^^^^^^^^ not a type

---

error[E0658]: language items are subject to change
  --> /checkout/src/test/ui/lang-items/issue-83471.rs:7:1
   |
LL | #[lang = "sized"]
   |
   = help: add `#![feature(lang_items)]` to the crate attributes to enable

error[E0658]: language items are subject to change
error[E0658]: language items are subject to change
  --> /checkout/src/test/ui/lang-items/issue-83471.rs:11:1
   |
LL | #[lang = "fn"]
   |
   = help: add `#![feature(lang_items)]` to the crate attributes to enable

warning: anonymous parameters are deprecated and will be removed in the next edition
---
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
   = note: `#[warn(anonymous_parameters)]` on by default

error[E0718]: `r#fn` language item must be applied to a trait with 1 generic argument
   |
   |
LL | #[lang = "fn"]
...
LL | trait Fn {
   |         - this trait has 0 generic arguments

