 rust
#![feature(rustc_macro, rustc_macro_lib)]

extern crate rustc_macro;

use rustc_macro::TokenStream;

#[rustc_macro_derive(Foo)]
pub fn derive_foo(input: TokenStream) -> TokenStream {
    input
}
