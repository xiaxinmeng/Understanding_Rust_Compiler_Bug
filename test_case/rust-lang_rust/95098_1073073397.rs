rust
impl<T: Clone, const N: usize> From<&[T; N]> for Vec<T> {…}
impl<T: Clone, const N: usize> From<&mut [T; N]> for Vec<T> {…}
