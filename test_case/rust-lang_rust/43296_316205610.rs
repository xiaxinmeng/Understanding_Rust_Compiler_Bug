rust
trait Foo: Iterator<Item = i32> {}
trait Bar: Iterator<Item = &static str> {}
