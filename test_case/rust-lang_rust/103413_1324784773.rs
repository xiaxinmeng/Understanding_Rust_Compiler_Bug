rust
use std::marker::PhantomData;

struct Foo<T>(Box<i32>, PhantomData<T>);

struct Bar<T>(T);
impl<T> Drop for Bar<T> {
    fn drop(&mut self) {}
}

fn main() {
    let _foo;
    let s = String::from("Hello, world!");
    _foo = helper(&*s);
}

fn helper<'a>(_: &'a str) -> Foo<Bar<&'a str>> {
    Foo(Box::new(1), PhantomData)
}
