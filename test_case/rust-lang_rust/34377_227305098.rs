 rust
pub trait Foo {
    type Type;
}

pub struct Bar(<Bar as Foo>::Type);

impl Foo for Bar {
    type Type = i8;
}

impl Copy for Bar {} //~ERROR the trait `Copy` may not be implemented
