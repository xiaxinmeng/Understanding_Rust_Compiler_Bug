rust
use serde::ser::Serialize;

struct BooperDoop;
type ScooperSwoop = *mut BooperDoop;

impl Serialize for ScooperSwoop {
    // ...
}

fn main() {}
