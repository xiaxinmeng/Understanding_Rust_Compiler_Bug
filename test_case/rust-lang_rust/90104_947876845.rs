plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no method named `skip_binder` found for reference `&Predicate<'tcx>` in the current scope
   --> compiler/rustc_middle/src/ty/mod.rs:495:14
    |
495 |         self.skip_binder().polarity
    |              ^^^^^^^^^^^ method not found in `&Predicate<'tcx>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
