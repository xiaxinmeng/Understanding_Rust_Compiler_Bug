
  tree fndecl = fn;
  if (TREE_CODE (fndecl) == ADDR_EXPR)
    fndecl = TREE_OPERAND (fndecl, 0);

if (TREE_CODE (fndecl) == FUNCTION_DECL && TREE_DEPRECATED(fndecl)) {
  RichLocation r(location);
  r.add_range(Location( DECL_SOURCE_LOCATION(fndecl)));
  rust_warning_at(r, "call to deprecated unction");
}
