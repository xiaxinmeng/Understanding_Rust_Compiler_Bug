rust
if let Some(def_id) = self.tables.opt_type_dependent_def_id(expr.hir_id) {
  // do something with `def_id`
} else {
  // do something else
}
