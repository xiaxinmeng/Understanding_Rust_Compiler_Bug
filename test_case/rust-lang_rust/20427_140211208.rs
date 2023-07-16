
// Assume `str` is in the prelude
use std::str; // Module `str` shadows `str` from the prelude
fn f(arg: &str) {} // Stops compiling, huge breakage
