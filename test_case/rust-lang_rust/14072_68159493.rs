 rust
pub struct Foo;
impl FromPrimitive for Foo {
    fn from_i64(_: i64) -> Option<Foo> { None }
    fn from_u64(_: u64) -> Option<Foo> { None}
}
