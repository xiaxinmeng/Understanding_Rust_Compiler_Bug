rust
use proc_macro::{Ident, Span, TokenStream};

#[proc_macro]
pub fn example(_: TokenStream) -> TokenStream {
    Ident::new("0_DISCRIMINANT", Span::call_site());
    TokenStream::new()
}                                                                                                   
