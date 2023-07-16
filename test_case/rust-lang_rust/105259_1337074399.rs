rust
impl<A, B> PartialOrd<&B> for &A where
    A: PartialOrd<B> + ?Sized,
    B: ?Sized,
