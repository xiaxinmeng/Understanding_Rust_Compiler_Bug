plain
normalized stderr:
warning: the feature `const_ptr_read` has been stable since 1.71.0-nightly and no longer requires an attribute to enable
  --> $DIR/issue-105821.rs:4:30
   |
LL | #![feature(adt_const_params, const_ptr_read, generic_const_exprs)]
   |
   = note: `#[warn(stable_features)]` on by default

warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-105821.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/issues/issue-105821.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-105821" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-105821/auxiliary"
stdout: none
warning: the feature `const_ptr_read` has been stable since 1.71.0-nightly and no longer requires an attribute to enable
  --> fake-test-src-base/const-generics/issues/issue-105821.rs:4:30
   |
   |
LL | #![feature(adt_const_params, const_ptr_read, generic_const_exprs)]
   |
   = note: `#[warn(stable_features)]` on by default

warning: 1 warning emitted
