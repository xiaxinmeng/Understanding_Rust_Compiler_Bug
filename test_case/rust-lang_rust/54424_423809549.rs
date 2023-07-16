rust
trait Foo {} // private trait!

impl<T> Foo for Cell<T>

enum MyOption<T: Foo> {
  Some(T), None
}
