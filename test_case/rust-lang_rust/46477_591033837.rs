rust
fn multiply3(
    iter: impl Iterator<Item = impl Into<f64>>,
    x: f64,
) -> impl Iterator<Item = f64> {
    iter.map(move |item| x * item.into())
}
