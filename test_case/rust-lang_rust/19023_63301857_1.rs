 rust
fn foo(s: Box<str>) {
    let s: &mut str = &mut *s;
}
