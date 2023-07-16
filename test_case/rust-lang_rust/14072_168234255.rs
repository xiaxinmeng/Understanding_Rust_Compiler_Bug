 rust
pub struct Foo;

pub trait Test: Sized {
    fn from_i64(_: i64) -> Option<Self>;
    fn from_u64(_: u64) -> Option<Self>;
}

impl Test for Foo {
    fn from_i64(_: i64) -> Option<Foo> { None }
    fn from_u64(_: u64) -> Option<Foo> { None}
}
