rust
use proc_macro::TokenStream;

#[proc_macro_derive(ToDiscriminant)]
pub fn derive_discriminant(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(TransitiveChild)]
pub fn derive_transitive_child(_: TokenStream) -> TokenStream {
    TokenStream::new()
}
