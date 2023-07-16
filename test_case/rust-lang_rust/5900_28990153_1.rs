
match variant.node.disr_expr {
    Some(e) => write_ty_to_tcx(tcx, e.id, result_ty),
    _ => {}
}
