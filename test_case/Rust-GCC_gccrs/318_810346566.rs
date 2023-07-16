c++
sexp to_sexp(Expr& expr) {
    sexp result;
    auto vis = begin_visitor<ASTVisitorBase>()
               .on<OperatorExpr>([](OperatorExpr& expr) {
                   sexpr lhs = ....;
                   sexpr rhs = ....;
                   result = sexp(expr.operator.to_string(), lhs, rhs);
               })
               .end_visitor();
    expr.accept_vis(vis);
    return result;
}
