rust
#![feature(specialization)]

trait PreProcess<T> {
  type Type: PostProcess<T> + std::default::Default;
  fn pre_process() -> Self::Type;
}

trait PostProcess<T> {
  fn post_process(self);
}

impl<T> PreProcess<T> for T {
  default type Type = std::marker::PhantomData<T>;
  default fn pre_process() -> Self::Type {
    return <Self::Type as std::default::Default>::default();
  }
}

pub struct Foo;

pub fn do_foo() {
  PostProcess::post_process(<Foo as PreProcess<Foo>>::pre_process());
}
