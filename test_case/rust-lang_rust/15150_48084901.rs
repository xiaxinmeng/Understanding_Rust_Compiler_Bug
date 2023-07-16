 rust
fn foo<'a>() {
    fn inner<'a>(x: &'a int) {}
}
