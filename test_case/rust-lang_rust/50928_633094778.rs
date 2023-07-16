
#[inline]
pub fn local_def_id_to_hir_id(&self, id: LocalDefId) -> hir::HirId {
    hir::HirId { owner: id, local_id: ItemLocalId::new(0) }
}
