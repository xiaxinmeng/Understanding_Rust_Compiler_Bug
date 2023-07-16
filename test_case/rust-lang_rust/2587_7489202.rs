
fn check_copy_ex(cx: ctx, ex: @expr, implicit_copy: bool) {
    if ty::expr_is_lval(cx.method_map, ex) &&
       !cx.last_use_map.contains_key(ex.id) &&
       !is_nullary_variant(cx, ex) {
        let ty = ty::expr_ty(cx.tcx, ex);
        check_copy(cx, ex.id, ty, ex.span, implicit_copy);
    }
}
