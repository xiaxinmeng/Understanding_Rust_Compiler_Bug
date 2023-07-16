rust
use std::fmt;

struct S;

impl fmt::Display for S {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        panic!("panic2")
    }
}

fn main() {
    panic!("panic1: {}", S)
}
