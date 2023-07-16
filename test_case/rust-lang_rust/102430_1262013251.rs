Rust
#![feature(dyn_star)]
#![allow(incomplete_features)]

use std::fmt::Display;

fn bug() -> impl Display {
    // This reference seems to be evil.
    // `1 as dyn* Display` compiles fine!
    &1 as dyn* Display
}

fn main() {
    let _ = bug();
}
