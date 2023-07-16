rust
#![feature(type_alias_impl_trait)]

trait Trait {}

type TAIT = impl Trait;

struct Bar;
impl Trait for Bar {}

fn foo() -> TAIT {
    Bar
}

fn haha() -> (impl Iterator<Item = TAIT>, impl Trait) {
    (std::iter::empty::<Bar>(), Bar)
}

fn bar() -> impl Iterator<Item = (TAIT, impl Trait)> {
    std::iter::once((foo(), Bar))
}
