rust
#[proc_macro]
#[uses(my_field)] // `quote!` will now expand `my_field` at the call site
fn example(_: TokenStream) -> TokenStream {
    quote! { #path { my_field: 0 } } // this just works now
}
