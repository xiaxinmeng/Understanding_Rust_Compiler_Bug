rust
impl<F: Fn(f64) -> f64 + Fn(f64, f64) -> f64> Integrand for F {
    // here you'd have to choose which "overload" to use if both exist
}
