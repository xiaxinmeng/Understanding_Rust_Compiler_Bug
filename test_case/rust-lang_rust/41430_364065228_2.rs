rust
#[proc_macro_attribute]
fn attribute(_: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::from_iter( input.into_iter() )
}
