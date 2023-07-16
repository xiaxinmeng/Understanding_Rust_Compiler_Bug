rust

impl<T, const N: usize> FromIterator<T> for Result<MyArray<T, N>, MyError> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iterator = iter.into_iter();
        ...
    }
}
