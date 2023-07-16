
mod m {
    pub fn f() {}
    pub mod n {}
}

use m::f::{self};
