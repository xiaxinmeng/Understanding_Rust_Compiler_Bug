rust
// Regression test for #69596. 
// TODO: Some explanation please :)

// revisions: rpass1 rpass2
// check-pass


#![allow(unused_imports)]

#[macro_export]
macro_rules! a_macro {
    () => {};
}

#[cfg(rpass1)]
use a_macro as same_name;

mod same_name {}

mod needed_mod {
    fn _crash() {
        use super::same_name;
    }
}   

fn main() {}
