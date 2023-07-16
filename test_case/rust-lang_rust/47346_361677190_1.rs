rust
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(MyCustomDerive)]
pub fn f(_input: TokenStream) -> TokenStream {
    "
        macro_rules! my_macro {
            () => {}
        }
    ".parse().unwrap()
}
