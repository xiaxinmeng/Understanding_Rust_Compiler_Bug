rust
trait Foo {
    type Bar;
}
trait Bar: Foo + AsRef<<Self as Bar>::Bar> {}
