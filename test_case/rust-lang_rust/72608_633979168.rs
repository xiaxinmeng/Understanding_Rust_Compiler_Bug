rust
#[proc_macro]
pub fn foo(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as syn::Expr);
    unimplemented!()
}
