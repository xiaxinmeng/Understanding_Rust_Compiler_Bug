 rust
trait Foo {
  fn answer(self);
}

struct NoData<T>;

impl<T> Foo for T
where NoData<T>: Foo {
  fn answer(self) {
    let val: NoData<T> = NoData;
  }
}


fn main() {}
