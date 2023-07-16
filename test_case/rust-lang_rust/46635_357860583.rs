rust
extern crate proc_macro2;
use proc_macro2::Span;

let span = Span::call_site();
let my_field = quote_spanned! {span=>
    my_field: 0
};
let result = quote! {
    #path { #my_field }
};
