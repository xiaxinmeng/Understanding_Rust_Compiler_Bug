rust
trait Bar {
    type Foo;
    // We want to default to just using Foo as its DetailedVariant
    type DetailedFoo = Self::Foo;

    // ...other trait methods...

    fn detailed(&self) -> DetailedFoo;

    // ...other trait methods...
}
