rust
#![feature(rust_2018_preview)]
#![feature(uniform_paths)]
use hex;
fn main() {
    println!("hex {}", hex::encode("01"));
}
