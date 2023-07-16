rust
#[proc_macro_attribute]
fn attribute(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}
