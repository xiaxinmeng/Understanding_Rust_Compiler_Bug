rust
use std::fmt::{self, Display};
use std::ops::Add;

struct X(usize);

impl Display for X {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("X")
    }
}

impl Add<X> for () {
    type Output = ();
    fn add(self, _: X) -> Self::Output {}
}

impl Drop for X {
    fn drop(&mut self) {
        print!("{}", self.0);
    }
}

fn main() {
    print!("{}", X(0)) + X(1);
}
