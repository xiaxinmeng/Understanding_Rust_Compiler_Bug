plain
    Checking smallvec v1.10.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#fe242b7e)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#fe242b7e)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0599]: no method named `span_fatal` found for reference `&CodegenCx<'gcc, 'tcx>` in the current scope
    |
    |
482 |             self.span_fatal(span, "failed to get layout for `{}`: {}", ty, err)
    |                  ^^^^^^^^^^ method not found in `&CodegenCx<'gcc, 'tcx>`
help: one of the expressions' fields has a method of the same name
    |
    |
482 |             self.tcx.sess.span_fatal(span, "failed to get layout for `{}`: {}", ty, err)

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_codegen_gcc` (lib) due to previous error
Build completed unsuccessfully in 0:01:40
