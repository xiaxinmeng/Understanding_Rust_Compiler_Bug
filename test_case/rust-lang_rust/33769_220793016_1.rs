 rust
impl <T: ::std::fmt::Debug + HasAssoc> ::std::fmt::Debug for Thing1<T> {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Thing1 { thing: ref __self_0_0 } => {
                let mut builder = __arg_0.debug_struct("Thing1");
                let _ = builder.field("thing", &&(*__self_0_0));
                builder.finish()
            }
        }
    }
}

impl <T: ::std::fmt::Debug + HasAssoc> ::std::fmt::Debug for Thing2<T> where
 T::Type: ::std::fmt::Debug {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Thing2 { thing: ref __self_0_0 } => {
                let mut builder = __arg_0.debug_struct("Thing2");
                let _ = builder.field("thing", &&(*__self_0_0));
                builder.finish()
            }
        }
    }
}
