rust
#![feature(nll)]
use std::ops::*;

#[derive(Copy, Clone)]
struct Au(u32);

impl Add<Au> for Au {
    type Output = Au;
    fn add(self, other: Au) -> Au {
        Au(self.0 + other.0)
    }
}

impl AddAssign<Au> for Au {
    fn add_assign(&mut self, other: Au) {
        *self = Au(self.0 + other.0)
    }
}

fn main() {
    let mut foo = vec![Au(4), Au(5), Au(6)];
    foo[2] += foo[2];
}
