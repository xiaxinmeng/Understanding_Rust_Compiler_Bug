plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0425]: cannot find value `else_blk` in this scope
    |
    |
484 |         let else_expr = self.arena.alloc(self.expr_block(else_blk, AttrVec::new()));

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_ast_lowering` due to previous error
warning: build failed, waiting for other jobs to finish...
