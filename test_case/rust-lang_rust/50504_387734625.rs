rust
// foo.rs
#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::*;

#[proc_macro_derive(Foo)]
pub fn foo1(_: TokenStream) -> TokenStream {
    "
    struct Outer;
    mod inner {
        type Inner = Outer; // `Outer` shouldn't be available from here
    }
    ".parse().unwrap()
}
