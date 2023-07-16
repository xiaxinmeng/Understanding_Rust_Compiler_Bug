rust
fn multiply<I>(iter: I, x: f64) -> impl Iterator<Item = f64>
where
    I: Iterator,
    I::Item: Into<f64>,
{
    iter.map(move |item| x * item.into())
}

fn multiply2<I>(iter: I, x: f64) -> impl Iterator<Item = f64>
where
    I: Iterator,
    I::Item: Into<f64>,
{
    fn mul<T: Into<f64>>(x: f64) -> impl Fn(T) -> f64 {
        move |item| x * item.into()
    }
    iter.map(mul(x))
}

fn iter() -> impl Iterator<Item = i32> {
    (0..10).map(|i| i * 42)
}

pub fn foo() {
    let _ = multiply(iter(), 2.0);
    let _ = multiply2(iter(), 2.0);
}
