rust
use core::cell::Cell;
//use core::marker::PhantomData as PDOrBox;
//use std::boxed::Box as PDOrBox;

type PDOrBox<T> = [T; 0];

struct Foo<'a> {
  selfref: Cell<Option<&'a Foo<'a>>>,
}
impl<'a> Drop for Foo<'a> {
  fn drop(&mut self) {
  }
}
fn make_selfref<'a>(x: &'a PDOrBox<Foo<'a>>) {}
fn make_pdorbox<'a>() -> PDOrBox<Foo<'a>> {
    unimplemented!()
}
fn main() {
  let x = make_pdorbox();
  make_selfref(&x);
}
