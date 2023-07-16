rust
struct MyType<T>(UnsafeCell<T>);

impl<T> MyType<T> {
  fn new() -> Self { ... }
  fn do(&mut self) { ... }
}
