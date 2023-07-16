rust
// src/lib.rs
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn print_input(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    println!("Input: {:?}", tokens);
    tokens
}
