rust
#[proc_macro]
fn m(input: TokenStream) -> TokenStream {
    quote! {
        extern crate my_crate;
        my_crate::my_function($input);
    }
}
