 rust
struct foo;
impl<'a> foo_FnOnce : FnOnce<()> for foo {
    type Output = &'a u32;
    ...
}
