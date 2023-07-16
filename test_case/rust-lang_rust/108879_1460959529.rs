plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0599]: no method named `root_const_var` found for reference `&InferCtxt<'tcx>` in the current scope
   --> compiler/rustc_trait_selection/src/solve/eval_ctxt.rs:120:35
    |
120 |                     && self.infcx.root_const_var(vid) == self.infcx.root_const_var(term_vid)
    |                                   ^^^^^^^^^^^^^^ help: there is a method with a similar name: `root_var`

error[E0599]: no method named `root_const_var` found for reference `&InferCtxt<'tcx>` in the current scope
   --> compiler/rustc_trait_selection/src/solve/eval_ctxt.rs:120:69
    |
120 |                     && self.infcx.root_const_var(vid) == self.infcx.root_const_var(term_vid)
    |                                                                     ^^^^^^^^^^^^^^ help: there is a method with a similar name: `root_var`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_trait_selection` due to 2 previous errors
