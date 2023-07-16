rust
impl<'a, 'tcx> LateLintPass<'a, 'tcx> for ChainedAssignment {
    fn check_expr(&mut self, cx: &LateContext<'_, '_>, e: &hir::Expr) {
        if let hir::ExprKind::Assign(_, ref right) = e.node {
            if let hir::ExprKind::Assign(..) = right.node {
                let msg = "chaining assignment is not supported in Rust";
                let cs = cx.tcx.sess.source_map().def_span(e.span);
                let mut err = cx.struct_span_lint(CHAINED_ASSIGNMENT, cs, msg);
                err.emit();
            }    
        }    
    }    
}
