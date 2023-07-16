`
#![feature(associated_consts)]

pub struct Foo(/* elided */);

impl Foo {
    pub const MY_CONST: &'static str = "hurr durr I'm a sheep";

    // Other defs elided
}
