rust
fn def_id_corresponds_to_hir_dep_node(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
    let hir_id = tcx.hir.as_local_hir_id(def_id).unwrap();
    def_id.index == hir_id.owner
}
