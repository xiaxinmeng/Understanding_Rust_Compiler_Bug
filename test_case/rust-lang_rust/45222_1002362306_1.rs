rust
#![feature(test)]
extern crate test;
use test::bench::black_box;

use num_integer::Roots;

fn main() {
    let v = 984067;

    for i in 1..=v {
        black_box(example(i, i.cbrt()));
    }

    for i in 1..=v {
        black_box(example_inc(i, i.cbrt()));
    }
}
