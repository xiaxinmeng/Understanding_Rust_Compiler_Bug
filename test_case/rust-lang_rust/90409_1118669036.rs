rust
#![feature(type_alias_impl_trait)]

trait Bar {
    fn bar(&self);
}

type FooFn<B> = impl FnOnce(B);

fn foo<B: Bar>() -> FooFn<B> {
    fn mop<B: Bar>(bar: B) { bar.bar() }
    mop as fn(B) // NOTE: function pointer
}

fn main() {
    let boom: FooFn<u32> = unsafe { core::mem::zeroed() };
    boom(42);
}
