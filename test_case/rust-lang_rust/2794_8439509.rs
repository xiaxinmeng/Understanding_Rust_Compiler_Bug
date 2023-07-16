
trait compiler_context {
    fn tcx() -> ty::ctxt;

    fn ty_to_str(t: ty::t) -> ~str { ty_to_str(self.tcx(), t) }
    fn expr_to_str(expr: @ast::expr) -> ~str { fmt!("expr(%?: %s)", expr.id, expr_to_str(expr, self.tcx().sess.intr()) }
    ...
}
