rust
#![feature(proc_macro)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Repro, attributes(repro))]
pub fn derive_repro(input: TokenStream) -> TokenStream {
    input.into_iter()
        .next()
        .unwrap()
        .span()
        .error("test")
        .emit();
    TokenStream::empty()
}
