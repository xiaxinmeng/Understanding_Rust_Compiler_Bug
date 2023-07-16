
sexp to_sexp(Expr& expr) {
    sexp result;
    auto vis = begin_visitor<ASTVisitorBase>()
               .on<OperatorExpr>([](OperatorExpr& expr) {
                   result = to_sexp(expr);
               })
               /* and other level 1 stuff */
               .end_visitor();
    expr.accept_vis(vis);
    return result;
}

sexp to_sexp(OperatorExpr& expr) {
    sexp result;
    auto vis = begin_visitor<ASTVisitorBase>()
               .on<ArithmeticOrLogicalExpr>([](ArithmeticOrLogicalExpr& expr) {
                   /* implement actual logic */
               })
               /* and other level 2 stuff */
               .end_visitor();
    expr.accept_vis(vis);
    return result;
}
