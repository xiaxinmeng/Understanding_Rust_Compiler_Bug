rust
if let Some(def) = self.tables.type_dependent_defs().get(expr.hir_id) {
  let def_id = def.def_id();
  // do something with `def_id`
} else {
  // do something else
}
