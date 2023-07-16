rust
fn visit_stmt(&mut self, stmt: &'tcx hir::Stmt) {
        debug!("visting statement {:#?}", stmt );
        self.check_stmt_attributes(stmt);
        intravisit::walk_stmt(self, stmt);
    }

    fn visit_expr(&mut self, ex: &'tcx hir::Expr) {
        debug!("visting an expr {:#?}", ex);
        intravisit::walk_expr(self,ex);
    }
