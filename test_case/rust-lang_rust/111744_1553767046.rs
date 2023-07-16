plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0046]: not all trait items implemented, missing: `make_memory_loop`
   --> src/builder.rs:400:1
    |
400 | impl<'a, 'gcc, 'tcx> BuilderMethods<'a, 'tcx> for Builder<'a, 'gcc, 'tcx> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `make_memory_loop` in implementation
    |
    = help: implement the missing item: `fn make_memory_loop<BodyPtrsVisitor>(&mut self, _: &str, _: [<Self as BackendTypes>::Value; VAR_COUNT], _: [rustc_target::abi::Size; VAR_COUNT], _: <Self as BackendTypes>::Value, _: BodyPtrsVisitor) where BodyPtrsVisitor: std::ops::FnMut, std::ops::FnOnce::Output = () { todo!() }`
For more information about this error, try `rustc --explain E0046`.
error: could not compile `rustc_codegen_gcc` (lib) due to previous error
Build completed unsuccessfully in 0:01:27
