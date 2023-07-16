rust
impl<T: Clone> From<&[T]> for Vec<T> {…}  // since 1.0.0
impl<T: Clone> From<&mut [T]> for Vec<T> {…}  // since 1.19.0
