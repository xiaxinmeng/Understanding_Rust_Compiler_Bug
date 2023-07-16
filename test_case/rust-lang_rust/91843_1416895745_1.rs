rust
// commented code does not compile
use core::num::Wrapping;
fn main() {
    let n = &mut [Wrapping(0)][..];
    n[0] += n[0];
    // std::ops::AddAssign::add_assign(&mut n[0], n[0]);
}
