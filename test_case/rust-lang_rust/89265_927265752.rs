rust
fn foo<'long: 'short, 'short>(value: (&'long str, i32)) -> impl Borrow<(&'short str, i32)> {
    value
}
