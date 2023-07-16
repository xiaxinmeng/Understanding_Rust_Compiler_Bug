rust
trait Foo<T: 'static> { .. }
struct Bar<T> { b: Box<dyn Foo<T>> }
