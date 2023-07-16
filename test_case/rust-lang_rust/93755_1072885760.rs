rust
impl<T, U, A> PartialEq<Vec<U, A>> for Vec<T, A>
where
    T: PartialEq<U>,
    A: Allocator;
