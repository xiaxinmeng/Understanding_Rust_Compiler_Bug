rust
trait Foo<T = i32> where Self: Sized, T: Into<Self> {}
