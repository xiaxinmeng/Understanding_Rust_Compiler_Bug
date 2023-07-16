 rust
#![crate_type = "lib"]

#![feature(globs)]

extern crate libc;

use std::mem;

use self::types::*;

pub mod types {
    use libc;
}

mod storage {
    use libc;
}

mod failing {
    use libc;
    use super::types::*;
}
