 rust
use std::marker::PhantomFn;
trait TTrans : PhantomFn<(Self)> { type O; }
type ApplyT<T> = <T as TTrans>::O;

trait Reusable<H> {
  type E;
  fn ap<R, F : Fn(Self::E)->R>(self, f : F) -> ApplyT<H>;
}

trait Other<H>    {
  type E;
  fn ap2<R, F : Fn(Self::E)->R>(self, f : F) -> ApplyT<H>;
}

impl<H,T> Reusable<H> for T where T : Other<H> , H : TTrans {
  type E = <Self as Other<H> >::E;
  fn ap<R, F : Fn(<Self as Other<H>>::E)->R>(self, f : F) -> ApplyT<H> {
    self.ap2(f)
  }
}

fn main() {}
