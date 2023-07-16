rust
trait Factory {
    type Item;
    fn produce(&mut self) -> Self::Item;
}

impl Factory for Foo {
    type Item = impl Debug;
    fn produce(&mut self) -> Self::Item {
        ()
    }
}
