 rust
fn foo<'a, 'b, T>() -> <() as Foo<'a, 'b, 'T>>::Type { ... }
trait Foo<'a, 'b, T> {
  type Type: Trait + 'static; // <-- note the `'static` bound appears here
}
impl<'a, 'b, T> Foo<...> for () {
  default type Type = /* something that doesn't reference `'a`, `'b`, or `T` */;
}
