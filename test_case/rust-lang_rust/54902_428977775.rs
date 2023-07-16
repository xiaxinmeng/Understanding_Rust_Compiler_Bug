rust
trait Foo<'a> { }

struct Bar<T> { t: T }

impl<T: Foo<'_>> Bar<T> { }

fn main() { }
