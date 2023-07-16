plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0599]: no method named `sess` found for mutable reference `&'b mut Resolver<'a>` in the current scope
     |
     |
2509 |                     if self.r.sess().features_untracked().non_lifetime_binders {
     |                               ^^^^ method not found in `&'b mut Resolver<'a>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_resolve` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_resolve` due to previous error
