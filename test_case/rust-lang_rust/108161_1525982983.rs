plain
........

failures:

---- [ui] tests/ui/const-generics/adt_const_params/const_param_ty_impl_bad_field.rs stdout ----

- error[E0204]: the trait `ConstParamTy` may not be implemented for this type
- error[E0204]: the trait `ConstParamTy` may not be implemented for this type
+ error[E0204]: the trait `ConstParamTy` cannot be implemented for this type
2   --> $DIR/const_param_ty_impl_bad_field.rs:10:36
3    |
4 LL | struct CantParam(NotParam);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/adt_const_params/const_param_ty_impl_bad_field/const_param_ty_impl_bad_field.stderr
To only update this specific test, also pass `--test-args const-generics/adt_const_params/const_param_ty_impl_bad_field.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/adt_const_params/const_param_ty_impl_bad_field.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/adt_const_params/const_param_ty_impl_bad_field" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/adt_const_params/const_param_ty_impl_bad_field/auxiliary"
stdout: none
error[E0204]: the trait `ConstParamTy` cannot be implemented for this type
error[E0204]: the trait `ConstParamTy` cannot be implemented for this type
  --> fake-test-src-base/const-generics/adt_const_params/const_param_ty_impl_bad_field.rs:10:36
   |
LL | struct CantParam(NotParam);
   |                  -------- this field does not implement `ConstParamTy`
LL |
LL | impl std::marker::ConstParamTy for CantParam {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0204`.
