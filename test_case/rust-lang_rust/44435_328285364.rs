rust
impl TyCtxt {
    fn in_scope_traits_of_expr(self, hir_id: HirId) -> Option<Rc<Vec<TraitCandidate>>> {
        // Get the map for the item containing the HirId. This is a query:
        let map_for_containing_item = self.in_scope_traits_of_item(hir_id.owner);
        // From that item-local map get actual return value
        map_for_containing_item.get(hir_id.local_id).cloned()
    }
}
