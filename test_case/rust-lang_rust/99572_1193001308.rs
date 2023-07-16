rust
use std::fmt;

struct BooperDoop;

impl fmt::Display for *mut BooperDoop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This not compile")
    }
}

fn main() {}
