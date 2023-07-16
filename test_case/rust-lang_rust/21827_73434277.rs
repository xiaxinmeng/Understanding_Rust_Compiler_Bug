 rust
#![feature(no_std)]
#![no_std]

extern crate "std" as other;
extern crate core;

fn main() {
    let i: &[i32] = &[0i32];
    for _ in i {}
}
