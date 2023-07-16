 rust
#[rustc_macro_derive(Identity)]
pub fn id(input: TokenStream) -> TokenStream {
    input
}
