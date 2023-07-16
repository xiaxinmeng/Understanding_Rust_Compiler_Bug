rust
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn wasm_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    let item = syn::parse::<syn::Item>(input).expect("parse");

    if let syn::Item::Fn(f) = item {
        f.to_tokens(&mut tokens);
    }

    tokens.into()
}
