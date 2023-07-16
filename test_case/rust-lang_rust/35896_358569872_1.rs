rust
#[proc_macro_derive(Foo)]
fn foo(_input: TokenStream) -> TokenStream {
    quote! {
        // Due to hygiene, these names never cause conflict errors.
        extern crate sql_macros;
        use sql_macros::sql;

        ... sql!(Struct) ...
    }
}
