rust
// src/lib.rs
#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn foo(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item.into_iter().collect()
}
