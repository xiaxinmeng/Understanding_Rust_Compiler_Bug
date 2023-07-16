rust
use proc_macro::TokenStream;

#[proc_macro_derive(Test)]
pub fn test(_: TokenStream) -> TokenStream {
    "{} ! @ # $ blåhaj % ^ &".parse().unwrap()
}
