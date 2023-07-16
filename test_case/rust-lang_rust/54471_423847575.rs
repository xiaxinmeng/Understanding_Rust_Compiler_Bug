rust
#![allow(unused)]

#[macro_use]
extern crate serde_derive;

use self::one::*;
use self::two::*;

mod serde {}

mod one {
    use serde;

    #[derive(Serialize)]
    #[serde]
    struct One;
}

mod two {
    use serde;

    #[derive(Serialize)]
    #[serde]
    struct Two;
}

fn main() {}
