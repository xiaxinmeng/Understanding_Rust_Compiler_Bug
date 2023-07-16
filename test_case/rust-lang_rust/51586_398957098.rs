rust
#![crate_type = "lib"]

struct A;
struct B;
trait Foo<T> {}
impl Foo<B> for A {}

fn f<T0>(_: T0) where A: Foo<T0> {
    g::<_>(B); // expected type `T0`
}

fn g<T1>(_: T1) where A: Foo<T1> {}
