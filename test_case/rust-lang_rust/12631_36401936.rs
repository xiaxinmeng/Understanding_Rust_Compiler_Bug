
macro_rules! impl_hash(
    ( $( $ty:ty => $method:ident;)* ) => (
        $(
            impl<S: Writer> Hash<S> for $ty {
                #[inline]
                fn hash(&self, state: &mut S) {
                    state.$method(*self);
                }
            }
        )*
    )
)
