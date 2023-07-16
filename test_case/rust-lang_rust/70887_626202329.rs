rust
use std::ops::{Div, Mul};
assert_eq!(5_u32.div(2).mul(2), 4);        // {mul, div} for ints
assert_eq!(5_f32.div(2.0).mul(2.0), 5.0);  // {mul, div} for floats
