rust
pub fn implement(attrs: Vec<Attribute>, data: DataStruct) -> TokenStream2 {
	let attrs: StructAttributes = parse_crate_attributes(&attrs).unwrap();
	super::common::implement_struct_body(attrs, data.fields, |ident| quote! { self.#ident })
}
