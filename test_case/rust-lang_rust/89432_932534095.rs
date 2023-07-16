rust
pub const fn foo() -> impl FnOnce() -> String {
    || "foo".into()
}
