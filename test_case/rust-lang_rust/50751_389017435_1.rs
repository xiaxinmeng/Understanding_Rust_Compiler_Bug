rust
impl EarlyLintPass for UnusedLoopLabel {
    fn check_expr(&mut self, _: &EarlyContext, expr: &ast::Expr) {
        // match `expr.kind`, looking for any of the loop kinds and pushing their `Label`s to `self.0`
        // also look for `Continue` or `Break`, and find/remove their labels from `self.0` in reverse order
        // (since loop labels can shadow each other we want to remove the innermost label as that's
        //  the one that'll be actually used)
        // use `label.ident.name` for comparison
    }
   
    // called after returning from recursing into nested expressions,
    //  this is where we emit the lint message
    fn check_expr_post(&mut self, ctxt: &EarlyContext, expr: &ast::Expr) {
        // look at loop kinds in `expr` again, if that loop's label is in `self.0` then emit a lint message like this:
        ctxt.span_lint(UNUSED_LOOP_LABEL, label.ident.span, "unused loop label");
        // (make sure to search in reverse order again, and also remove the label from the vec)
    }
}
