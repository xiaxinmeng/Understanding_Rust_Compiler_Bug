 rust
fn leak<'a, T>(x: T) -> &'a T {
    (&x).clone()
}
