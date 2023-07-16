rust
#![feature(associated_type_defaults)]
#![feature(associated_consts)]

trait Foo {
    type Bar;
}

trait Baz {
    type Fv: Foo = u8;
    const C: <Self::Fv as Foo>::Bar = 6665;
}
