rust
fn f() -> impl for<'a> Fn(&'a u8) -> (impl Debug + 'a) {
    |x| x
}
