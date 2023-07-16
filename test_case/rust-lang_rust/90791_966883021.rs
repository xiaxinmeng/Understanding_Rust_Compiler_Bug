plain
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking object v0.25.3
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0046]: not all trait items implemented, missing: `const_i16`, `const_i64`
   |
   |
89 | impl<'gcc, 'tcx> ConstMethods<'tcx> for CodegenCx<'gcc, 'tcx> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `const_i16`, `const_i64` in implementation
   |
   = help: implement the missing item: `fn const_i16(&self, _: i16) -> <Self as BackendTypes>::Value { todo!() }`
   = help: implement the missing item: `fn const_i64(&self, _: i64) -> <Self as BackendTypes>::Value { todo!() }`
For more information about this error, try `rustc --explain E0046`.
error: could not compile `rustc_codegen_gcc` due to previous error
Build completed unsuccessfully in 0:03:34
