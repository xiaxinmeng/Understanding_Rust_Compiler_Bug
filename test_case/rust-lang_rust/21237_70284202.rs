 rust
struct Foo<T>(T);
impl<T> Clone for Foo<T> where T: Clone { ... }

pub struct Bar<T>(Foo<T>);
impl<T> Clone for Bar<T> where Foo<T>: Clone { ... }
