rust
struct Foo<T: Cake>(T);

impl<T> Lie for Foo<Wrapping<T>> where Wrapping<T>: Cake {}
