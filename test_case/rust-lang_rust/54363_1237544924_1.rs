rust
// src/lib.rs

use proc_macro::TokenStream;

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let serde /*: proc_macro::Ident */ = proc_macro::dependency("serde");
    quote! {
        impl #serde::Serialize for …
    }
}
