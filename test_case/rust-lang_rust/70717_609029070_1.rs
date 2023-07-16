rust
use quote::quote;

pub fn repro() {
    let many = [1, 2, 3];

    let _together = quote! {
        #(#many),*
    };
}
