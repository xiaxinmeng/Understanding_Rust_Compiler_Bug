
#[link(name = "foo",
       vers = "0.1")];
#[crate_type = "lib"];

pub trait Foo {
    static pub fn foo() -> self;
}

pub impl int: Foo {
    static pub fn foo() -> int { 42 }
}
