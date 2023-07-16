rust
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Example, attributes(example::attr))]
pub fn example_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
