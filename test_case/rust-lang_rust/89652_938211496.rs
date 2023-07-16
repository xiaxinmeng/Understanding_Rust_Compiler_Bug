plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0046]: not all trait items implemented, missing: `type_metadata`, `typeid_metadata`
   --> src/builder.rs:385:1
    |
385 | impl<'a, 'gcc, 'tcx> BuilderMethods<'a, 'tcx> for Builder<'a, 'gcc, 'tcx> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `type_metadata`, `typeid_metadata` in implementation
    |
    = help: implement the missing item: `fn type_metadata(&mut self, _: <Self as BackendTypes>::Function, _: std::string::String) { todo!() }`
    = help: implement the missing item: `fn typeid_metadata(&mut self, _: std::string::String) -> <Self as BackendTypes>::Value { todo!() }`
error[E0046]: not all trait items implemented, missing: `type_test`
  --> src/intrinsic/mod.rs:76:1
   |
   |
76 | impl<'a, 'gcc, 'tcx> IntrinsicCallMethods<'tcx> for Builder<'a, 'gcc, 'tcx> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `type_test` in implementation
   |
   = help: implement the missing item: `fn type_test(&mut self, _: <Self as BackendTypes>::Value, _: <Self as BackendTypes>::Value) -> <Self as BackendTypes>::Value { todo!() }`
For more information about this error, try `rustc --explain E0046`.
error: could not compile `rustc_codegen_gcc` due to 2 previous errors
Build completed unsuccessfully in 0:03:36
