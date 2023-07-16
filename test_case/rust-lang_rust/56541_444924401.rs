rust
/// Returns `true` if `def_id` is a trait alias.
pub fn is_trait_alias(tcx: TyCtxt<'_, '_, '_>, def_id: DefId) -> bool {
    if let Some(node_id) = tcx.hir.as_local_node_id(def_id) {
        if let Node::Item(item) = tcx.hir.get(node_id) {
            if let hir::ItemKind::TraitAlias(..) = item.node {
                return true;
            }
        }
    }
    false
}
