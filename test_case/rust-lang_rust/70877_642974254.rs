rust
#![feature(type_alias_impl_trait)]

pub trait Baz { }

impl Baz for () { }

type Qux = impl Baz;

pub trait Foo {
    type Assoc;
}

impl Foo for () {
    type Assoc = Qux;
}

type Bar = impl Foo<Assoc = Qux>;

fn assign() -> Bar {}

fn assign2() -> Qux {}

extern "C" {
    pub fn lint_me() -> <Bar as Foo>::Assoc;
}

fn main() {}
