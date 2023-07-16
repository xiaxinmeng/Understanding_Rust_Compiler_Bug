rust
fn adt_destructor(tcx: TyCtxt<'_>, def_id: DefId) -> Option<ty::Destructor> {
    tcx.calculate_dtor(def_id, &mut dropck::check_drop_impl)
}
