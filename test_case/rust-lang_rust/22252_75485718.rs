 rust
#![feature(unsafe_destructor)]

struct Foo;
struct FooRef<'a>(&'a Foo);

impl<'a> FooRef<'a> {
    fn borrow(&self) {}
}

#[unsafe_destructor]
impl<'a> Drop for FooRef<'a> {
    fn drop(&mut self) {}
}

fn main() {
    let x = Foo;
    FooRef(&x).borrow()
}
