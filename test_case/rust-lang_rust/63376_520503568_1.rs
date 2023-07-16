rust
trait Foo<'a> {
  type Item: 'a + ?Sized;
}
