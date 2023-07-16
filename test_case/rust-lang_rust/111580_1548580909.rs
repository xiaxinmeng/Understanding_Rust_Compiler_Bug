plain
    Checking smallvec v1.10.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#fe242b7e)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#fe242b7e)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0433]: failed to resolve: use of undeclared crate or module `ssa_errors`
    |
    |
482 |             self.tcx.sess.emit_fatal(ssa_errors::FailedToGetLayout { span, ty, err })
    |                                      ^^^^^^^^^^ use of undeclared crate or module `ssa_errors`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_codegen_gcc` (lib) due to previous error
Build completed unsuccessfully in 0:01:38
