 Rust
extern crate num;
use std::i64::MIN;
use num::rational::Ratio;

let a = Ratio::new(MIN, 1);
a.abs() == a; // true
