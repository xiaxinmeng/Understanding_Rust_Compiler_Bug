
#[link(name = "foo",
       vers = "0.1")];
#[crate_type = "lib"];

pub use sub_foo::Foo;

pub mod sub_foo {    // <- added `pub` to this line and got the same error when compiling bar.rs
    pub trait Foo {
        static pub fn foo() -> self;
    }

    pub impl int: Foo {
        static pub fn foo() -> int { 42 }
    }
}
