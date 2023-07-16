rust
#![feature(proc_macro)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(AsChangeset, attributes(table_name))]
pub fn derive(input: TokenStream) -> TokenStream {
    for tt in input {
        println!("{:#?}", tt);
    }
    TokenStream::empty()
}
