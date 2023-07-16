rust
pub fn derive(input: DeriveInput) -> TokenStream2 {
	let name = input.ident;
	let generics = add_trait_bounds(input.generics);
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let implementation = implement(input.attrs, input.data);

	quote! {
		impl #impl_generics ::encde::Encode for #name #ty_generics #where_clause {
			fn encode(&self, writer: &mut dyn std::io::Write) -> ::encde::Result<()> { // is this the issue?
				#implementation
				Ok(())
			}
		}
	}
}
