 rust
// foo.rs
extern crate num;

use num::bigint::BigInt;
use std::num::One;

fn main() {
    let a: BigInt = One::one();

    println!("{}", a.to_str());
}
