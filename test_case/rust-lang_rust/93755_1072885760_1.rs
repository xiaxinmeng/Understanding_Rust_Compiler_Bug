rust
impl<T, U, A1, A2> PartialEq<Vec<U, A2>> for Vec<T, A1>
where
    T: PartialEq<U>,
    A1: Allocator,
    A2: Allocator;
