rust
// src/lib.rs
use neon::types::Value;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn proc(attr: TokenStream, item: TokenStream) -> TokenStream {
    panic!()
}
