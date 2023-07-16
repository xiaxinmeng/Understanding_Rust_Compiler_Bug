plain
diff of stderr:

11   --> $DIR/feature-gate-rustdoc_internals.rs:7:1
12    |
13 LL | #[doc(tuple_variadic)]
+    | ^^^^^^^^^^^^^^^^^^^^^^
15    |
16    = note: see issue #90418 <https://github.com/rust-lang/rust/issues/90418> for more information
17    = help: add `#![feature(rustdoc_internals)]` to the crate attributes to enable
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-rustdoc_internals.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-rustdoc_internals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-rustdoc_internals" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-rustdoc_internals/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: `#[doc(keyword)]` is meant for internal use only
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL | #[doc(keyword = "match")] //~ ERROR: `#[doc(keyword)]` is meant for internal use only
   |
   = note: see issue #90418 <https://github.com/rust-lang/rust/issues/90418> for more information
   = help: add `#![feature(rustdoc_internals)]` to the crate attributes to enable


error[E0658]: `#[doc(tuple_variadic)]` is meant for internal use only
   |
   |
LL | #[doc(tuple_variadic)]  //~ ERROR: `#[doc(tuple_variadic)]` is meant for internal use only
   |
   = note: see issue #90418 <https://github.com/rust-lang/rust/issues/90418> for more information
   = help: add `#![feature(rustdoc_internals)]` to the crate attributes to enable

