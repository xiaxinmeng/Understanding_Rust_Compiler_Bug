plain
error[E0382]: assign to part of moved value: `err`
   --> compiler/rustc_borrowck/src/diagnostics/region_errors.rs:535:17
    |
496 |         let mut err =
    |             ------- move occurs because `err` has type `FnMutError`, which does not implement the `Copy` trait
...
499 |         let mut diag = self.infcx.tcx.sess.create_err(err);
    |                                                       --- value moved here
...
535 |                 err.upvar_def_span = Some(upvar_def_span);

    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustc_borrowck` due to previous error
