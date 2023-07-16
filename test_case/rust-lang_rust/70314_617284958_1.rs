rust
#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn include_v1(_input: TokenStream) -> TokenStream {
    let ident = syn::Ident::new("V1__initial", proc_macro2::Span::call_site());
    let result = quote::quote! { pub mod #ident; };
    result.into()
}
