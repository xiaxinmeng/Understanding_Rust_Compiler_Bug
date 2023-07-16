rust
trait Foo {
    fn dynamic(&self) -> &dyn Foo where Self: SIzed { self as &dyn Foo }
}

impl dyn Foo {
    fn dynamic(&self) -> &dyn Foo { self }
}
