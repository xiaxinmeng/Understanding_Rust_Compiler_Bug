rust
#![feature(use_extern_macros)]

use std::panic::{self};

fn main() {
    assert!(true); //~ ERROR expected (), found bool
}
