
// For a constant body, there are no inputs, and one
// "output" (the type of the constant).
DefiningTy::Const(def_id, substs) => {
    let ty = tcx.type_of(def_id);
    let ty = indices.fold_to_region_vids(tcx, &ty); // equivalently, `ty.subst(substs)` would work
    ty::Binder::dummy(tcx.mk_type_list(iter::once(ty)))
}
