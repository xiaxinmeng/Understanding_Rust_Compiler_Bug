rust
  struct Foo<T>;
  impl<T> Foo<T> {
    fn bar<U>(&self) {}
  }

  let x: Foo<u32> = ...;
  x.bar(false);
  