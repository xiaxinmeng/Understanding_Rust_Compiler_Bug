extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rename_params(args: TokenStream, input: TokenStream) -> TokenStream {
    dbg!(&input);
    input
}
