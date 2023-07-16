plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0046]: not all trait items implemented, missing: `type_test`
  --> src/intrinsic/mod.rs:76:1
   |
76 | impl<'a, 'gcc, 'tcx> IntrinsicCallMethods<'tcx> for Builder<'a, 'gcc, 'tcx> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `type_test` in implementation
   |
   = help: implement the missing item: `fn type_test(&mut self, _: <Self as BackendTypes>::Value, _: <Self as BackendTypes>::Value) -> <Self as BackendTypes>::Value { todo!() }`
error[E0308]: mismatched types
   --> src/builder.rs:922:55
    |
    |
922 |     fn typeid_metadata(&mut self, _typeid: String) -> Self::Value {
    |        ---------------                                ^^^^^^^^^^^ expected struct `RValue`, found `()`
    |        |
    |        implicitly returns `()` as its body has no tail or `return` expression
Some errors have detailed explanations: E0046, E0308.
For more information about an error, try `rustc --explain E0046`.
error: could not compile `rustc_codegen_gcc` due to 2 previous errors
Build completed unsuccessfully in 0:03:40
