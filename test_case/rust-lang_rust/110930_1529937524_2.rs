
use std::mem::ManuallyDrop;
use std::ops::Drop;

#[inline(always)]
fn must_be_inlined() {
    let _s = "this should be inlined";
}

struct Foo;

struct Bar {
    a: u32,
}

impl Drop for Bar {
    fn drop(&mut self) {
        must_be_inlined()
    }
}

impl A for Foo {
    type MyType = Bar;
}

trait A {
    type MyType;
}

fn func<T: A>(foo: T, arg: <T as A>::MyType) {
    let mut to_drop = unsafe { ManuallyDrop::new(arg) };
    unsafe { ManuallyDrop::drop(&mut to_drop) }
}

fn main() {
    let bar = Bar { a: 1 };
    let foo = Foo {};
    func(foo, bar);
}
