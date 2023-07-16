rust
fn multiply3<I, T>(iter: I, x: f64) -> impl Iterator<Item = f64>
where
    I: Iterator<Item = T>,
    T: Into<f64>,
{
    iter.map(move |item| x * item.into())
}
