rust
let opt_expr = match self.tcx.hir.find(cause.body_id) {
    rustc::hir::map::NodeExpr(e) => Some(e),
    _ => None,
}
