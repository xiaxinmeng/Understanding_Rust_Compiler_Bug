plain
4 LL | #[derive(std::marker::ConstParamTy)]
5    |          ^^^^^^^^^^^^^^^^^^^^^^^^^
6 
- error: aborting due to previous error
+ error[E0277]: the type `Union` does not `#[derive(PartialEq)]`
+    |
+ LL | impl std::marker::ConstParamTy for Union {}
+ LL | impl std::marker::ConstParamTy for Union {}
+    |                                    ^^^^^ the trait `StructuralPartialEq` is not implemented for `Union`
+ note: required by a bound in `ConstParamTy`
+   --> $SRC_DIR/core/src/marker.rs:LL:COL
8 
+ error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args const-generics/adt_const_params/const_param_ty_impl_union.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/adt_const_params/const_param_ty_impl_union.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/adt_const_params/const_param_ty_impl_union" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/adt_const_params/const_param_ty_impl_union/auxiliary"
stdout: none
error: this trait cannot be derived for unions
  --> fake-test-src-base/const-generics/adt_const_params/const_param_ty_impl_union.rs:18:10
   |
LL | #[derive(std::marker::ConstParamTy)]
LL | #[derive(std::marker::ConstParamTy)]
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the type `Union` does not `#[derive(PartialEq)]`
  --> fake-test-src-base/const-generics/adt_const_params/const_param_ty_impl_union.rs:16:36
LL | impl std::marker::ConstParamTy for Union {}
LL | impl std::marker::ConstParamTy for Union {}
   |                                    ^^^^^ the trait `StructuralPartialEq` is not implemented for `Union`
note: required by a bound in `ConstParamTy`
  --> /rustc/FAKE_PREFIX/library/core/src/marker.rs:1003:1

error: aborting due to 2 previous errors
