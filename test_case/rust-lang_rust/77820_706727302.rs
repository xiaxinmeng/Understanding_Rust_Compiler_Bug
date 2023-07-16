rust
impl Clean<Item> for doctree::Struct<'_> {
    fn clean(&self, cx: &DocContext<'_>) -> Item {
        Item::from_hir_id_and_kind(
            self.id,
            StructItem(Struct {
                struct_type: self.struct_type,
                generics: self.generics.clean(cx),
                fields: self.fields.clean(cx),
                fields_stripped: false,
            }),
            cx,
        )
    }
}
