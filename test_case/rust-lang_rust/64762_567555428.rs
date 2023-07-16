rust
#![deny(unreachable_pub)]

mod m1 {
    pub mod m2 {
        pub use super::Item1;
    }
    
    pub struct Item1;
}

pub use m1::m2::*;
