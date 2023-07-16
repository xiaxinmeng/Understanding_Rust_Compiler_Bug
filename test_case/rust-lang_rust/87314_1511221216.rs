rs
use ::proc_macro::*;

#[proc_macro_derive(Foo)] pub
fn __(_input: TokenStream)
  -> TokenStream
{
    stringify!(
        enum Foo {};

        impl ::core::fmt::Display for Foo {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str("Foo")
            }
        }
    )
    .parse()
    .unwrap()
}
