rust
impl<P, U> CoerceUnsized<Pin<Box<U>>> for Pin<Box<P>> where P: CoerceUnsized<U> {}
impl<P, U> CoerceUnsized<Pin<Box<U>>> for Pin<Box<P>> where P: CoerceUnsized<U> {}
impl<P, U> CoerceUnsized<Pin<Cell<U>>> for Pin<Cell<P>> where P: CoerceUnsized<U> {}
...
