rust
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn foo(_input: TokenStream, _attributes: TokenStream) -> TokenStream {
    quote! {
        pub enum Foo {
            Bar,
            Baz,
        }

        pub fn bar(foo: Foo) {
            match foo {
                Bar => {},
                Baz => {},
            }
        }
    }.into()
}
