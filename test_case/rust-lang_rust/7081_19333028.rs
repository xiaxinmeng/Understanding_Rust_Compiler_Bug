
trait Visitor {
    fn visit_expr(&mut self, expr: @ast::expr) {
        super_expr(self, expr);
    }
    fn visit_fn(&mut self, ...);
}

// It occurs to me that we have to break "super functions" into their own
// top-level functions if we want them to be callable.
fn super_expr<V:Visitor>(
    this: &mut V,
    expr: @ast::expr)
{
        match expr.node {
            ast::expr_binary(op, left, right) => {
                self.visit_expr(left); self.visit_expr(right);
            }
            ...
        }
}                                  
