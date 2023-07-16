 Rust
extern crate foo;
extern crate bar;

pub trait Baz : foo::Foo + bar::Bar {}
pub type BazT = Baz<FooT=(), BarT=u32>;
