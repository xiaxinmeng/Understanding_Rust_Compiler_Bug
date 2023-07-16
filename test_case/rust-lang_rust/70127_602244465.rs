rust
fn concrete<'a>(f: impl Fn(&'a u8)) -> impl Fn(&'a u8) {
    f
}

fn bound(f: impl Fn(&u8)) -> impl Fn(&u8) {
    concrete(f)
}
