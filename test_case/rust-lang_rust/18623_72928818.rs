 rust
use std::ops::Add;

trait Scalar {}
impl Scalar for f64 {}

struct Bob;

impl<RHS: Scalar> Add<RHS> for Bob {
    type Output = Bob;
    fn add(self, rhs: RHS) -> Bob { self }
}

fn main() {
    let b = Bob;
    b + 3.5;
    // Internal compiler error (should be type error?):
    b + 3;
}
