 rust
trait A {}
trait B {}

trait Foo<T> { } 

impl<T, U> Foo<T> for U: A { ... }
impl<T, U> Foo<T> for U: B { ... }
