rust 
// lib.rs
use proc_macro::TokenStream;

use syn::spanned::Spanned;
use syn::{parse_macro_input, Result};
use syn::parse::{Parse, ParseStream};
use quote::quote;

struct Export {
    item: syn::Item,
    ident: syn::Ident,
}

impl Parse for Export {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Export {
            item: input.parse()?,
            ident: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn export(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Export);
    let mut ident = input.ident;
    let span = ident.span().resolved_at(input.item.span());
    ident.set_span(span.into());
    quote!(#ident).into()
}
