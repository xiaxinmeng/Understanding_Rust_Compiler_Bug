Rust


struct A;
struct B;

trait Foo<'a> {
  type Bar;
}

impl<'a> Foo<'a> for B {
  type Bar = A;
}

trait Bar: for<'a> Foo<'a> {
  fn f<F: for<'a> FnOnce(<Self as Foo<'a>>::Bar)>(&mut self, f: F) {}
}

impl Bar for B {}

fn main() {
  B.f(drop);
}
