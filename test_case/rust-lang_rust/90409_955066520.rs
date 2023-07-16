rust
#![feature(type_alias_impl_trait)]

trait Bar {
    fn bar(&self);
}

trait Baz {
    fn baz(&self);
}

struct MyBaz<B: Bar>(B);
impl <B: Bar> Baz for MyBaz<B> {
    fn baz(&self) {}
}

type FooFn<B> = impl Baz;

fn foo<B: Bar>(bar: B) -> FooFn<B> {
    MyBaz(bar)
}

fn main() {
    let boom: FooFn<u32> = unsafe { core::mem::zeroed() };
    boom.baz();
}
