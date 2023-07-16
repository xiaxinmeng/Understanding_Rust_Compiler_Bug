rust
#[proc_macro]
fn m(_: TokenStream) -> TokenStream {
    quote! {
        extern crate foo; // due to hygiene, this is never a conflict error
        foo::f();
        // --- or just --- (if/when we get the sugar)
        $universe::foo::f();
    }
}
