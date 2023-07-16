rust
impl<'a, T: 'a + Copy> Extend<&'a T> for Vec<T> {
