c++
sexp to_sexp(Expr& expr) {
    sexp result;
    auto vis = begin_visitor<ASTVisitorBase>()
               .on<OperatorExpr>([](OperatorExpr& expr) {
                    sexpr lhs = to_sexpr(expr.lhs);
                    sexpr rhs = to_sexpr(expr.rhs);
                   result = sexp(expr.operator.to_string(), lhs, rhs);
               })
               .end_visitor();
    expr.accept_vis(vis);
    return result;
}
