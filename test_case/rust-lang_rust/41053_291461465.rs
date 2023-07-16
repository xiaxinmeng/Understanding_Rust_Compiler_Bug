Rust
pub trait Trait { fn foo(&self) {} }
pub struct Foo;

impl Iterator for Foo {
    type Item = ();
    fn next(&mut self) -> Option<()> {
        extern crate bar;
        impl ::Trait for bar::S {
            fn foo(&self) {}
        }
        None
    }
}
