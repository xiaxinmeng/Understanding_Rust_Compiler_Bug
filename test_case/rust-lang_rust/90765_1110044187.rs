rs
#[proc_macro]
pub fn expand_into(stream: TokenStream) -> TokenStream {
    let mut parts = stream.into_iter();
    let Some(TokenTree::Group(unexpanded)) = parts.next() else { panic!() };
    let into = parts.collect::<TokenStream>();
    let expanded = unexpanded.expand_expr().unwrap();
    quote!(#into!(#expanded)).into()
}
