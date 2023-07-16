rust
#[proc_macro_attribute]
pub fn my_attr(_args: TokenStream, input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as syn::ItemMod);
    let content = module.content.unwrap().1; // workaround for mod having no name
    proc_macro::TokenStream::from(quote! { #(#content)* })
}
