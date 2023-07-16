rust
// foo.rs
#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro::*;

#[proc_macro_derive(Foo)]
pub fn foo1(a: TokenStream) -> TokenStream {
    drop(a);
    "mod __bar { static mut BAR: Option<Something> = None; }".parse().unwrap()
}
