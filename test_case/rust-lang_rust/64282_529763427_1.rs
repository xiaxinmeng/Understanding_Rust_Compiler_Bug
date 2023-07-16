rust
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rename_params(args: TokenStream, _: TokenStream) -> TokenStream {
    dbg!(&args);
    TokenStream::new()
}
