rust
mod m {
    fn f() {}
}

use m::f; // Doesn't import anything and therefore reports an error.
use m::g; // Doesn't import anything and therefore reports an error.
use m::*; // Doesn't import anything and therefore reports an error.
