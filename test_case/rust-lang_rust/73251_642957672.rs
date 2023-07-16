rust
#![feature(type_alias_impl_trait)]

pub trait Foo {
    type Assoc;
}

impl Foo for () {
    type Assoc = u32;
}

type Bar = impl Foo<Assoc = u32>;

fn assign() -> Bar {}

extern "C" {
    pub fn lint_me() -> <Bar as Foo>::Assoc;
}

