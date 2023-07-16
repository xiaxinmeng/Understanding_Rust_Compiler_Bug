 rust
struct Data;

trait Foo<T> {}
trait Bar<T> {}

impl<T, U> Bar<U> for T    where T: Foo<U> {}
impl<U>    Bar<U> for Data {}
