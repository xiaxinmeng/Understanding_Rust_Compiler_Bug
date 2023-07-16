plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0046]: not all trait items implemented, missing: `align_metadata`
   --> src/builder.rs:385:1
    |
385 | impl<'a, 'gcc, 'tcx> BuilderMethods<'a, 'tcx> for Builder<'a, 'gcc, 'tcx> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `align_metadata` in implementation
    |
    = help: implement the missing item: `fn align_metadata(&mut self, _: <Self as BackendTypes>::Value, _: Align) { todo!() }`
For more information about this error, try `rustc --explain E0046`.
error: could not compile `rustc_codegen_gcc` due to previous error
Build completed unsuccessfully in 0:03:48
