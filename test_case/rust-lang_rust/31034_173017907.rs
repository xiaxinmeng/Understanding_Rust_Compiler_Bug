 rust
trait Mul<RHS = Self> {
    type Output: Zero;
}

trait Zero {
    fn zero() -> Self;
}

fn main() {
    <f64 as Mul>::Output::zero();
}
