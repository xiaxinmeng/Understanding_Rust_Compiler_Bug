rust
use crate::sub::*;

mod sub {
    // This, whether from a glob import or regular import, hides the extern crate `num`
    pub mod num {
        pub fn one() -> u64 {
            2
        }
    }
}

// This hides both the extern crate `num` and `crate::sub::num` coming from glob import
mod num {
    pub fn one() -> u64 {
        3
    }
}

fn main() {
    let x: u64 = num::one();
    println!("x: {:?}", x);
}
