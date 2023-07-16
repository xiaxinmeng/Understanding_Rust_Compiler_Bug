rust
impl <const N : usize, T: ::core::fmt::Debug> ::core::fmt::Debug for
 Example<T, N> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Example(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Example");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
