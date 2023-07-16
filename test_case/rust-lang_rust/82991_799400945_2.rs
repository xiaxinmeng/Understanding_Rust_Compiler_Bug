rust
use proc_macro::TokenStream;

#[proc_macro]
pub fn foo(_arg: TokenStream) -> TokenStream {
    TokenStream::new()
}
