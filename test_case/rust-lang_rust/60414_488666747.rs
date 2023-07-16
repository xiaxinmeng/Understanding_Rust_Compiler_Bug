rust
#![feature(impl_trait_in_bindings)]

struct Foo;

trait FooLike { type Output; }
impl FooLike for Foo {
    type Output = u32;
}

trait Trait {
    type Assoc;
}

fn foo<T: Trait<Assoc = i32>>() {
    let _: impl FooLike<Output = T::Assoc> = Foo;
}
