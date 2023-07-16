rust
let _has_own_dtor = |ty: Ty<'_>| match ty.kind() {
    ty::Adt(adt, _) => self.fcx.tcx.adt_destructor(adt.did()).is_some(),
    _ => false,
};
