rust
trait Foo<'a> { }

struct Bar<T> { t: T }

impl<T> Bar<T> where T: Foo<'_> { }

fn main() { }
