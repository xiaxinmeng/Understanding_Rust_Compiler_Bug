rust
fn retrieve_primitive_defid_if_needed(tcx: TyCtxt<'_>, cache: &Cache, def_id: DefId) -> DefId {
    if matches!(tcx.def_kind(def_id), DefKind::Mod) &&
        let Some(prim) = tcx.get_attrs(def_id, sym::doc)
            .flat_map(|attr| attr.meta_item_list().unwrap_or_default())
            .filter(|attr| attr.has_name(sym::primitive))
            .find_map(|attr| attr.value_str()) {
        crate::clean::PrimitiveType::from_symbol(prim).and_then(|p| crate::clean::Type::Primitive(p).def_id(cache)).unwrap_or(def_id)
    } else {
        def_id
    }
}

// Then we call it into `JsonRenderer::convert_item`:
impl JsonRenderer<'_> {
    pub(super) fn convert_item(&self, item: clean::Item) -> Option<Item> {
        let deprecation = item.deprecation(self.tcx);
        let links = self
            .cache
            .intra_doc_links
            .get(&item.item_id)
            .into_iter()
            .flatten()
            .map(|clean::ItemLink { link, page_id, fragment, .. }| {
                let id = match fragment {
                    Some(UrlFragment::Item(frag_id)) => {
                        // We need this work-around because links to primitive.
                        retrieve_primitive_defid_if_needed(self.tcx, &self.cache, *frag_id)
                    },
                    // FIXME: Pass the `UserWritten` segment to JSON consumer.
                    Some(UrlFragment::UserWritten(_)) | None => {
                        // We need this work-around because links to primitive.
                        retrieve_primitive_defid_if_needed(self.tcx, &self.cache, *page_id)
                    }
                };
                (link.clone(), from_item_id(id.into(), self.tcx))
            })
            .collect();
