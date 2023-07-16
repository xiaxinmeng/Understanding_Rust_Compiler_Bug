rust
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Beta, attributes(foo))]
pub fn beta_derive(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);

    TokenStream::new()
}
