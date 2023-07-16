rust
struct CustomWrapper<T>(T);

impl<T> Into<T> for CustomWrapper<T> {
    fn into(self) -> T { self.0 }
}
