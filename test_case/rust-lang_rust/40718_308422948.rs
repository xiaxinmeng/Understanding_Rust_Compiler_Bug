rust
#![feature(specialization)]

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

trait Tag {}
struct TagA;
struct TagB;
struct TagC;
impl Tag for TagA {}
impl Tag for TagB {}
impl Tag for TagC {}

struct Foo<T: Tag> { data: [i32; 8], _pd: PhantomData<T> }

impl<T: Tag> Deref for Foo<T> {
    type Target = [i32];
    default fn deref(&self) -> &Self::Target { &self.data[..7] }
}
impl<T: Tag> DerefMut for Foo<T> {
    default fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data[..7] }
}

impl Deref for Foo<TagA> {
    fn deref(&self) -> &Self::Target { &self.data[1..] }
}
impl DerefMut for Foo<TagA> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data[1..] }
}

trait FooBuilder: Sized
  + DerefMut<Target = [i32]> // ***
{
    fn new_foo(data: [i32; 8]) -> Self;
}
impl<T: Tag> FooBuilder for Foo<T> {
    fn new_foo(data: [i32; 8]) -> Self { Self { data, _pd: PhantomData } }
}

struct Bar<T: Tag>(Foo<T>);
impl<T: Tag> Bar<T> {
    fn new(data: [i32; 8]) -> Self { Bar(Foo::new_foo(data)) } // !!!
}

fn main() {
    let data = [0i32; 8];
    let _: Bar<TagA> = Bar::new(data);
    let _: Bar<TagB> = Bar::new(data);
    let _: Bar<TagC> = Bar::new(data);
}
