rust

#[proc_macro_derive(Foo)]
pub fn foo(input: TokenStream) -> TokenStream {
    ...
}
#[proc_macro_derive(Bar, depends_on(Foo))]
pub fn bar(input: TokenStream) -> TokenStream {
    ...
}
