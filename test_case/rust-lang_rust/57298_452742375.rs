rust
/// Validate that the given HirId (respectively its `local_id` part) can be
/// safely used as a key in the tables of a TypeckTable. For that to be
/// the case, the HirId must have the same `owner` as all the other IDs in
/// this table (signified by `local_id_root`). Otherwise the HirId
/// would be in a different frame of reference and using its `local_id`
/// would result in lookup errors, or worse, in silently wrong data being
/// stored/returned.
fn validate_hir_id_for_typeck_tables(local_id_root: Option<DefId>,
                                     hir_id: hir::HirId,
                                     mut_access: bool) {
    if cfg!(debug_assertions) {
        if let Some(local_id_root) = local_id_root {
            if hir_id.owner != local_id_root.index {
                ty::tls::with(|tcx| {
                    let node_id = tcx.hir().hir_to_node_id(hir_id);

                    bug!("node {} with HirId::owner {:?} cannot be placed in \
                          TypeckTables with local_id_root {:?}",
                         tcx.hir().node_to_string(node_id),
                         DefId::local(hir_id.owner),
                         local_id_root)
                });
            }
        } else {
            // We use "Null Object" TypeckTables in some of the analysis passes.
            // These are just expected to be empty and their `local_id_root` is
            // `None`. Therefore we cannot verify whether a given `HirId` would
            // be a valid key for the given table. Instead we make sure that
            // nobody tries to write to such a Null Object table.
            if mut_access {
                bug!("access to invalid TypeckTables")
            }
        }
    }
}
