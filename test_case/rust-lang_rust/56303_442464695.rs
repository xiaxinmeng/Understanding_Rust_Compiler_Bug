rust
// Crate 1
pub mod m {
    pub use foo as _;
}

pub use m::*;

// Crate 2
use crate1::m::*;
use crate1::*;
