rust
trait Foo<'a> {
  type Bar;
}

trait Bar: for<'a> Foo<'a> {
  fn f<F>(&mut self, f: F)
  where
    F: for<'a> FnOnce(<Self as Foo<'a>>::Bar) -> <Self as Foo<'a>>::Bar,
  {
  }
}

fn main() {}
