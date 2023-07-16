rust
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Testing)]
pub fn derive_uri_display(input: TokenStream) -> TokenStream {
    println!("Tokens: {:?}", input);
    TokenStream::new()
}
