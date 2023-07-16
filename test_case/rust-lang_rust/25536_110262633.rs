 rust
trait Foo<T> {}

fn bar<T>(t: Foo<&T>) -> Foo<&T> { t }
