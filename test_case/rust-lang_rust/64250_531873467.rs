rust
LateLintPass::check_struct_field(.., cx, field);
cx.tables.qpath_res(field.ty.node -> qpath, field.ty.hir_id);
