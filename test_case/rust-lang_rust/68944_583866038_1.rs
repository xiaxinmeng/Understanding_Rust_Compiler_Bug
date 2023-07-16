rust
pub struct IndexedHir<'hir> {
    /// The SVH of the local crate.
    pub crate_hash: Svh,

    pub(super) owner_map: FxHashMap<DefIndex, &'hir HirOwner<'hir>>,
    pub(super) owner_items_map: FxHashMap<DefIndex, &'hir HirOwnerItems<'hir>>,

    /// The reverse mapping of `node_to_hir_id`.
    pub(super) hir_to_node_id: FxHashMap<HirId, NodeId>,
}

pub struct HirOwner<'tcx> {
    parent: HirId,
    node: Node<'tcx>,
}

pub struct HirItem<'tcx> {
    parent: ItemLocalId,
    node: Node<'tcx>,
}

pub struct HirOwnerItems<'tcx> {
    owner: Node<'tcx>,
    items: IndexVec<ItemLocalId, Option<HirItem<'tcx>>>,
    bodies: FxHashMap<ItemLocalId, &'tcx Body<'tcx>>,
}
