rust
trait Foo {
    type Bar: Iterator<Item = impl Display>
}
