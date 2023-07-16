rust
// Does not compile
pub fn foo2(mut a: &mut Demo) {
    if let Some(ref mut b) = &mut a.foo {
        a = b;
    }
    a.foo = None;
}
