rust
    pub fn is_assign_to_bool(&self, expr: &hir::Expr, expected: Ty<'tcx>) -> bool {
        if let hir::ExprKind::Assign(..) = expr.node {
            return expected == self.tcx.types.bool;
        }
        false
    }
