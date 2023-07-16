 rust
trait Bar {}
struct Foo<T = usize>(T) where T: Bar;
