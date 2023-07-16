rust
use lib::{Foreign, B};

struct Local;  // (Bool)

impl B<Local> for Foreign {}

fn main() {}
