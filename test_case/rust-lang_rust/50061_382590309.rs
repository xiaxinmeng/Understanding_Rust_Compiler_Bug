rust
// foo.rs
#![crate_type = "proc-macro"]
#![feature(proc_macro)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn foo(_a: TokenStream, b: TokenStream) -> TokenStream {
    b.into_iter().collect()
}
