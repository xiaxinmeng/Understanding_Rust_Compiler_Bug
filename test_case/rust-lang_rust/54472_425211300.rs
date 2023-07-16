rust
use std::*;

fn main() {
    macro_rules! try { ($expr:expr) => () }
    
    try!(0);
}
