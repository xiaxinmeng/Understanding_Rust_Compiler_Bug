 rust
trait Signed {
    fn abs(&self) -> Self;
}

impl Signed for float {
    fn abs(&self) -> float { ... }
}

fn main() {
    let a = float::abs(3.0);
}
