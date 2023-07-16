rust
 self.with_catch_scope(body.id, |this| {
    let mut block = this.lower_block(body, true);
    block.expr = wrap_in_from_ok_call(block.expr);
    hir::ExprBlock(block)
})
