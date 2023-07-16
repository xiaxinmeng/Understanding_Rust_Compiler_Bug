rust
impl<A, B> IntoFuture for (A, B) where
    A: IntoFuture,
    B: IntoFuture<Error = A::Error>, 
