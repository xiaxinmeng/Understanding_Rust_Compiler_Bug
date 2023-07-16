
#[link(name = "m")]
extern {
    fn tgamma(x: f64) -> f64;
}
fn gamma(x: f64) -> f64 {
    unsafe { tgamma(x) }
}
