rust
fn mixed<'a>(f: impl Fn(&u8)) -> impl Fn(&'a u8) {
    concrete(f)
}
