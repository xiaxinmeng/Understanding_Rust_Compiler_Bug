rust
struct Foo<T>(T, &mut *const T);
impl<T> Drop for Foo<T> {
    fn drop(&mut self) {
        *self.1 = &self.0 as *const T;
    }
}
