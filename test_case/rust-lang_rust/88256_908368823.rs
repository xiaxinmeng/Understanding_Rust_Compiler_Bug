rust
fn visit_expr(&mut self, e: &'a ast::Expr) {
        self.with_lint_attrs(e.id, &e.attrs, |cx| {
            run_early_pass!(cx, check_expr, e);       // C1: create fir the diagnostic 
            ast_visit::walk_expr(cx, e);                     // C2: create the second diagnostic
        })
    }
