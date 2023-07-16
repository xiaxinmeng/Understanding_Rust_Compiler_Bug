 rust
#![feature(macro_rules)]
// This should be required
// #[feature(log_syntax)];

macro_rules! hey { () => { log_syntax!() } }

fn main() {
    hey!()                                                                                         
}

