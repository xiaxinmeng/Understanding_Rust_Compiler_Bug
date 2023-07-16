rust

trait Integrand {
    ...
}

impl<F> Integrand for Fn(f64) -> f64 {
    ...
}

impl<F> Integrand for Fn(f64, f64) -> f64 {
    ...
}
