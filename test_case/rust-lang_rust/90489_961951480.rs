rust
    fn visit_foreign_item(&mut self, item: &ast::ForeignItem) {
        self.load_links_in_attrs(&item.attrs, item.span);
        visit::walk_foreign_item(self, item)
    }

    // NOTE: if doc-comments are ever allowed on function parameters, this will have to implement `visit_param` too.

    fn visit_assoc_item(&mut self, item: &ast::AssocItem, ctxt: visit::AssocCtxt) {
        self.load_links_in_attrs(&item.attrs, item.span);
        visit::walk_assoc_item(self, item, ctxt)
    }

    fn visit_field_def(&mut self, field: &ast::FieldDef) {
        self.load_links_in_attrs(&field.attrs, field.span);
        visit::walk_field_def(self, field)
    }

    fn visit_variant(&mut self, v: &ast::Variant) {
        self.load_links_in_attrs(&v.attrs, v.span);
        visit::walk_variant(self, v)
    }
}
