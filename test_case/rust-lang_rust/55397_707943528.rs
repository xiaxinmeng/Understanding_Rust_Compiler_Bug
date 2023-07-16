rust
extern crate proc_macro;
use proc_macro::*;

#[proc_macro]
pub fn name(input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}
