 rust
pub fn calc<X>() -> f64 where f64: Mul<X, Output=X> {
    <f64 as Mul<f64>>::mul(2.0f64, 3.0f64)
}
