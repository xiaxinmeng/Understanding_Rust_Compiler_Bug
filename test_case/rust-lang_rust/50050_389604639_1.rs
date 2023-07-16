rust
#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use] extern crate quote;

use syn::Ident;
use proc_macro::{Span, TokenStream};
use quote::Tokens;

fn create(ident: &Ident) -> Tokens {
    // // This works.
    // quote!(do_f!(#ident))

    // // This does as well.
    // let new_ident = Ident::from(ident.to_string());
    // quote!(do_f!(#new_ident))

    // // So does this.
    // quote!(do_f!(x))

    // // This does not.
    // let source = format!("do_f!({})", ident);
    // let expr: syn::Expr = syn::parse_str(&source).unwrap();
    // quote!(#expr)

    // // Neither does this.
    // let source = format!("do_f!({})", ident);
    // let stream: proc_macro2::TokenStream = source.parse().unwrap();
    // quote!(#stream)

    // Nor this.
    let source = format!("do_f!({})", ident);
    let stream: proc_macro2::TokenStream = source.parse().unwrap();
    quote_spanned!(Span::call_site() => #stream)
}

#[proc_macro_attribute]
pub fn demo(_args: TokenStream, _input: TokenStream) -> TokenStream {
    let ident = Ident::from("x");
    let expr = create(&ident);
    let tokens: Tokens = quote! {
        macro_rules! do_f {
            ($id:ident) => ($id + 1)
        }

        fn f(#ident: usize) -> usize {
            #expr
        }
    };

    tokens.into()
}
