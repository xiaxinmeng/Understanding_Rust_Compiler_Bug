rust

struct Foo<T>(PhantomData<T>);

impl<T> Trait for Foo<T> {
  type X = T;
}
