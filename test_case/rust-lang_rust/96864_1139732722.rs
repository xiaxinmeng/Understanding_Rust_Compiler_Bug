rust
pub(crate) fn is_parent_a_mod(&self) -> bool {
    let def_id = match self.item_id {
        DefId(def_id) => def_id,
        Auto { .. } | Blanket { .. } => return true,
        ItemId::Primitive(_, _) => return false,
    };
    if let Some(def_id) = def_id.as_local() {
        let hir = tcx.hir();
        let parent_def_id = hir.get_parent_item(hir.local_def_id_to_hir_id(def_id));

        matches!(hir.opt_def_kind(parent_def_id), Some(DefKind::Mod))
    } else {
        false
    }
}

pub(crate) fn has_page(&self, tcx: TyCtxt<'_>) -> bool {
    !self.is_stripped() && !self.is_import() && parent.is_mod() && self.full_name().is_some()
}
