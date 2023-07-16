rust
macro_rules! m {
    () => {
        extern crate core as std;
    }
}

m!();

use std::mem;

fn main() {}
