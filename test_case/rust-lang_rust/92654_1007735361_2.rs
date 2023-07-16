rust
fn implement(attrs: Vec<Attribute>, data: Data) -> TokenStream2 {
	match data {
		Data::Struct(data) => struct_impl::implement(attrs, data),
		Data::Enum(data) => enum_impl::implement(attrs, data),
		Data::Union(_) => unimplemented!(),
	}
}
