rust
pub fn foo() -> &'static dyn Sync {
    &None::<Foo>
}
