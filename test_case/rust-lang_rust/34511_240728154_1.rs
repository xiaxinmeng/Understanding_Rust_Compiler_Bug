 rust
fn foo<'a, 'b, T>() -> <() as Foo<...>>::Type { ... }
trait Foo<...> {
  type Type: Trait;
}
impl<...> Foo<...> for () {
  default type Type = /* inferred */;
}
