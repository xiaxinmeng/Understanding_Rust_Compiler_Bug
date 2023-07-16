rust
    fn filter_map_expr(&mut self, expr: P<ast::Expr>) -> Option<P<ast::Expr>> {
        let expr = configure!(self, expr);
        expr.filter_map(|mut expr| {
