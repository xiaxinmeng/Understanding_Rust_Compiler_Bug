rust
Dump::visit (LiteralExpr &literal_expr)
{
  stream << literal_expr.get_literal ().as_string () << " "
	 << literal_expr.get_mappings ().as_string ();
}
