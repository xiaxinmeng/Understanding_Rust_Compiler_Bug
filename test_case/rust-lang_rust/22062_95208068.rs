
use std::ops::DerefMut;

struct Foo<T>
where T: DerefMut
{
  member: *mut T::Target,
}

trait Bar<T>
where T: DerefMut
{
  fn foo<'a>(&'a self) -> &'a Foo<T>;
  fn member<'a>(&'a self) -> &'a *mut T::Target {
    &self.foo().member
  }
}
