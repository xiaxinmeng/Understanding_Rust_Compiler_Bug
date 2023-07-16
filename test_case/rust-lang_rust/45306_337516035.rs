rust
impl<'a, T> From<&'a T> for &'a [T] { ... }
impl<'a, T> From<&'a mut T> for &'a mut [T] { ... }
