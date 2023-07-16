 rust
    ExprKind::Tup(ref elts) => {
        hir::ExprTup(elts.iter().map(|x| self.lower_expr(x)).collect())
    } 
