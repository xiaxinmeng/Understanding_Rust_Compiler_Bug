Rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput};

#[proc_macro_derive(Bar)]
pub fn derive_bar(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    match data {
        Data::Enum(DataEnum { variants, .. }) => {
            let constants = variants.iter().map(|variant| {
                quote! {
                    const #variant: () = ();
                }
            });
            quote! {
                impl #ident {
                    #(#constants)*
                }
            }
            .into()
        }
        _ => panic!(),
    }
}
