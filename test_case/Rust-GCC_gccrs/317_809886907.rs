c++
void
TypeCheckExpr::visit (HIR::BlockExpr &expr)
{
  TyTy::BaseType *block_tyty
    = new TyTy::TupleType (expr.get_mappings ().get_hirid ());

  expr.iterate_stmts ([&] (HIR::Stmt *s) mutable -> bool {
    bool is_final_stmt = expr.is_final_stmt (s);
    bool has_final_expr = expr.has_expr () && expr.tail_expr_reachable ();
    bool stmt_is_final_expr = is_final_stmt && !has_final_expr;

    auto resolved = TypeCheckStmt::Resolve (s, inside_loop);
    if (resolved == nullptr)
      {
	rust_error_at (s->get_locus_slow (), "failure to resolve type");
	return false;
      }

    if (stmt_is_final_expr)
      {
	delete block_tyty;
	block_tyty = resolved;
      }
    else if (!resolved->is_unit ())
      {
	rust_error_at (s->get_locus_slow (), "expected () got %s",
		       resolved->as_string ().c_str ());
      }

    return true;
  });

  if (expr.has_expr ())
    {
      delete block_tyty;

      block_tyty
	= TypeCheckExpr::Resolve (expr.get_final_expr ().get (), inside_loop);
    }

  infered = block_tyty->clone ();
}
