rust
pub struct Foo(!);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Foo(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Foo");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                                                   ^^^^^^^^^^^^^^
                                                   warning: unreachable expression
                debug_trait_builder.finish()
            }
        }
    }
}
