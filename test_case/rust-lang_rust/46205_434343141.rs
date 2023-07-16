rust
struct CustomWrapper<T>(T);

impl<T> From<CustomWrapper<T>> for T {
    fn from(w: CustomWrapper<T>) -> T { w.0 }
}
