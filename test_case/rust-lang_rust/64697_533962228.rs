rust
pub trait Foo<T> {
    fn foo() -> &'static fn(T);
}

fn foo(_: &i32) {}

struct A;

impl<'a> Foo<&'a i32> for A {
    fn foo() -> &'static fn(&'a i32) {
        &(foo as _)
    }
}

fn bar<'a>(x: &'a i32, _: A) {
    A::foo()(x);
}
