plain
    Checking tracing-tree v0.2.0
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0615]: attempted to take value of method `ident` on type `&rustc_middle::ty::FieldDef`
    --> src/librustdoc/clean/mod.rs:1615:36
     |
1615 |         clean_field(self.did, self.ident.name, cx.tcx.type_of(self.did).clean(cx), cx)
     |
help: use parentheses to call the method
     |
     |
1615 |         clean_field(self.did, self.ident(_).name, cx.tcx.type_of(self.did).clean(cx), cx)


error[E0615]: attempted to take value of method `ident` on type `&VariantDef`
    --> src/librustdoc/clean/mod.rs:1686:64
     |
1686 |             Item::from_def_id_and_parts(self.def_id, Some(self.ident.name), VariantItem(kind), cx);
     |
help: use parentheses to call the method
     |
     |
1686 |             Item::from_def_id_and_parts(self.def_id, Some(self.ident(_).name), VariantItem(kind), cx);


error[E0615]: attempted to take value of method `ident` on type `VariantDef`
    --> src/librustdoc/html/render/print_item.rs:1779:57
     |
1779 |                         let ident = adt.variants[index].ident;
     |
help: use parentheses to call the method
     |
     |
1779 |                         let ident = adt.variants[index].ident(_);


error[E0615]: attempted to take value of method `ident` on type `&rustc_middle::ty::FieldDef`
    |
    |
390 |                         if def.all_fields().any(|item| item.ident.name == variant_field_name) {
    |
help: use parentheses to call the method
    |
    |
390 |                         if def.all_fields().any(|item| item.ident(_).name == variant_field_name) {


error[E0615]: attempted to take value of method `ident` on type `&&rustc_middle::ty::FieldDef`
    |
    |
711 |                     .find(|item| item.ident.name == item_name)?;
    |
help: use parentheses to call the method
    |
    |
711 |                     .find(|item| item.ident(_).name == item_name)?;


error[E0615]: attempted to take value of method `ident` on type `&rustc_middle::ty::FieldDef`
    |
    |
714 |                     UrlFragment::StructField(field.ident.name),
    |
help: use parentheses to call the method
    |
    |
714 |                     UrlFragment::StructField(field.ident(_).name),


error[E0615]: attempted to take value of method `ident` on type `&VariantDef`
     |
     |
2238 |             (parent_def, Some(UrlFragment::Variant(variant.ident.name)))
     |
help: use parentheses to call the method
     |
     |
2238 |             (parent_def, Some(UrlFragment::Variant(variant.ident(_).name)))

For more information about this error, try `rustc --explain E0615`.
error: could not compile `rustdoc` due to 7 previous errors
Build completed unsuccessfully in 0:02:40
