rust
#![feature(proc_macro)]
extern crate proc_macro;
use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro]
pub fn add(token_stream: TokenStream) -> TokenStream {
    TokenStream::from_str(&token_stream.to_string().replace(",", "+")).unwrap()
}
