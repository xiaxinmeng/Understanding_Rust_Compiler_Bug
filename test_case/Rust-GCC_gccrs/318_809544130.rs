c++
sexp to_sexp(Expr& expr) {
    sexp result;
    auto vis = begin_visitor<ASTVisitorBase>()
               .on<ArrayExpr>([&result](ArrayExpr& arr) {
                   result = sexp{"ArrayExpr", {"elements", arr.get_array_elements()}};
               })
               .on<TupleExpr>([&result](TupleExpr& tuple) {
                   result = sexp{"TupleExpr", tuple.get_tuple_elems()};
               })
               /* and many others */
               .end_visitor();
    expr.accept_vis(vis);
    return result;
}
