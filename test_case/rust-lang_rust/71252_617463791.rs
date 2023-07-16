rust
let tables = tcx.typeck_tables_of(tcx.hir().local_def_id(hir_id));
let opt_ty = tables.node_type_opt(expr.hir_id));
