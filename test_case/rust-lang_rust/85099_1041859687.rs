rust
impl<P, U> CoerceUnsized<Pin<U>> for Pin<P> where P: CoerceUnsized<U> {}
