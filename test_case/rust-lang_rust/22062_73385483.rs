 rust
use std::ops::{Deref, DerefMut};
use std::ptr;

struct F<'a> {
    x: &'a mut i32
}

impl<'a> Deref for F<'a> {
    type Target = i32;

    fn deref<'b>(&'b self) -> &'b i32 {
        self.x
    }
}

impl<'a> DerefMut for F<'a> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut i32 {
        self.x
    }
}

struct Foo<T>
    where T: DerefMut
{
   member: *mut <T as DerefMut>::Target,
}

trait Bar<T>
    where T: 'static + DerefMut
{
    fn foo<'a>(&'a self) -> &'a Foo<T>;

    fn member<'a>(&'a self) -> &'a *mut <T as DerefMut>::Target {
        let x = self.foo();
        &x.member
    }
}

struct FooContainer<T> where T: 'static + DerefMut {
    foo: Foo<T>
}

impl<T> Bar<T> for FooContainer<T> where T: 'static + DerefMut {
    fn foo<'a>(&'a self) -> &'a Foo<T> {
        &self.foo
    }
}

fn main() {
    let foocontainer: FooContainer<F> = FooContainer { foo: Foo { member: ptr::null_mut() } };
}
