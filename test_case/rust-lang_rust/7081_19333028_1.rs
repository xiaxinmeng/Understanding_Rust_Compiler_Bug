
struct MyContext {
    tcx: ty::ctxt, ...
}

impl visit::Visitor for MyContext {
    fn visit_expr(&mut self, expr: @ast::expr) {
        visit::super_expr(self, expr);
    }
}
