rust
#![feature(string_remove_matches)]

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut s = ("a".repeat(5000) + "b").repeat(5000);
    s.remove_matches('b');
    println!("{:?}", start.elapsed());
}
