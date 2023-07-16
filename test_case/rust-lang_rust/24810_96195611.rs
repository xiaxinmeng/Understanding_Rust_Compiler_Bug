 rust
use std::ops::Add;

trait T: Add {
}

impl<X: T> T<Output=X> {
}

fn main() {
}
