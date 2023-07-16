rust
trait Foo: AsRef<Self::Bar> {
    type Bar;
}
trait Bar: Foo + AsRef<Self::Bar> {}
