plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0609]: no field `sess` on type `&mut LoweringContext<'hir>`
   |
49 | ...                   self.sess
   |                            ^^^^ unknown field
   |
   |
   = note: available fields are: `tcx`, `resolver`, `arena`, `bodies`, `attrs` ... and 20 others
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_ast_lowering` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_ast_lowering` due to previous error
