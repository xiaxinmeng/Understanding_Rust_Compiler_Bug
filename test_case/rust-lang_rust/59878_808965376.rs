rust
// library/alloc/src/boxed.rs

#[stable(feature = "boxed_slice_into_iter", since = "1.53.0")]
impl<T, A> IntoIterator for Box<[T], A> {
    type Item = T;
    type IntoIter = crate::vec::IntoIter<T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

#[stable(feature = "boxed_array_into_iter", since = "1.53.0")]
impl<T, A, const N: usize> IntoIterator for Box<[T; N], A> {
    type Item = T;
    type IntoIter = crate::vec::IntoIter<T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
