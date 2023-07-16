 rust
trait Marker {}
struct Bob;
impl Marker for Bob {}

trait DoNothing {
    type Output;
}
impl DoNothing for Bob {
    type Output = Bob;
}
struct Dude<M: Marker>(M);

// Conflicting implementation with this line uncommented,
// Marker not implemented for <Bob as DoNothing>::Output error with it commented
impl Marker for <Bob as DoNothing>::Output {}

const b: Dude<<Bob as DoNothing>::Output> = Dude(Bob);

fn main() {
}
