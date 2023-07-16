
use std::marker::PhantomData;

trait Foo {
    type A;
    type B;
    fn foo(&self, x: Self::A) -> Self::B;
}

struct Bar<T>(PhantomData<T>);

impl<T: 'static> Foo for Bar<T> {
    type A = &'static T;
    type B = &'static T;
    fn foo(&self, x: Self::A) -> Self::B {
        x
    }
}

fn bad<'a, T>(x: &Foo<A = &'a T, B = &'static T>, k: &'a T) -> &'static T {
    x.foo(k)
}

/// Extends the lifetime of an arbitrary reference.
fn extend<'a, T>(x: &'a T) -> &'static T {
    bad(&Bar(PhantomData), x)
}

fn dangle_ref() -> &'static String {
    let x = "hello world".to_string();
    extend(&x)
}

fn main() {
    // This segfaults.
    println!("{}", dangle_ref());
}

