rust
#![feature(test)]

extern crate test;

#[test] fn test_something() { test::black_box(1); }

fn main() { /* no test functionality used */ }
