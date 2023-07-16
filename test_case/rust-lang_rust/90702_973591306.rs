rust
match self.r.crate_loader.process_extern_crate(
    item,
    &self.r.definitions,
    local_def_id,
) {
    Ok(crate_id) => {
        self.r.extern_crate_map.insert(local_def_id, crate_id);
        self.r.expect_module(crate_id.as_def_id())
    }
    Err(err) => {
        err.report(&self.r.session);
        let fake = self.r.arenas.alloc_import(Import {
            kind: ImportKind::ExternCrate { source: orig_name, target: ident },
            root_id: item.id,
            id: item.id,
            parent_scope: self.parent_scope,
            imported_module: Cell::new(None),
            has_attributes: !item.attrs.is_empty(),
            use_span_with_attributes: item.span_with_attributes(),
            use_span: item.span,
            root_span: item.span,
            span: item.span,
            module_path: Vec::new(),
            vis: Cell::new(vis),
            used: Cell::new(true),
        });
        self.r.import_dummy_binding(fake);
        return;
    }
}
