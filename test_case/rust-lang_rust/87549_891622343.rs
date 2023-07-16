rust
pub struct Table<T, const N: usize>([Option<T>; N]);

impl<'a, T, const N: usize> IntoIterator for &'a Table<T, N> {
    type IntoIter = ::core::iter::Flatten<::core::slice::Iter<'a, T>>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}
