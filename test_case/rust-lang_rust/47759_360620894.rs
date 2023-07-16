rust
struct Foo;
trait Wat {
    fn exp(self) -> Foo;
}
impl Wat for f32 {
    fn exp(self) -> Foo { Foo }
}
impl std::ops::Mul<Foo> for f64 {
    type Output = &'static str;
    fn mul(self, _: Foo) -> &'static str {
        "wat"
    }
}

fn main() {
    let _ = |x: f64| x * 2.0.exp();
    //let _ = |x: f64| x * 2.0f64.exp();
}
