rust
impl<P, U, A> CoerceUnsized<Pin<Box<U, A>>> for Pin<Box<P, A>>
where
    P: ?Sized + Unsize<U>,
    U: ?Sized,
    A: Allocator
{}
