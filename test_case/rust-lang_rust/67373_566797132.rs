rust
// A proc-macro crate named "pmf".
extern crate proc_macro;
extern crate syn;
extern crate quote;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Foo, attributes(helper))]
pub fn foo(tokens: TokenStream) -> TokenStream {
    let s = syn::parse_macro_input!(tokens as syn::DeriveInput);
    let helper = s.attrs.iter().filter(|attr| attr.path.is_ident("helper")).next().unwrap();
    let name = match helper.parse_meta().unwrap() {
        syn::Meta::NameValue(v) => v.lit,
        _ => panic!("expected helper=lit"),
    };
    let name_str = match name {
        syn::Lit::Str(s) => s,
        _ => panic!("expected string"),
    };
    let ty: syn::Type = name_str.parse().unwrap();  // KEY POINT HERE
    let ts = quote!{
        impl Foo for #ty {}
    };
    TokenStream::from(ts)
}
