 rust
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Foo {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Foo::Bar(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Bar");
                builder.field(&&(*__self_0));
                builder.finish()
            }
        }
    }
}
