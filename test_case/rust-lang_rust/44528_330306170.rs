rust
#[proc_macro_derive(Foo, attributes(foo))]
fn derive_foo(input: TokenStream) -> TokenStream { input }
