 rust
extern mod extra;

use std::vec;

#[bench]
fn test1(b: &mut extra::test::BenchHarness) {
    do b.iter {
        vec::from_fn(1000, |_| 0);
    }
}

#[bench]
fn test2(b: &mut extra::test::BenchHarness) {
    do b.iter {
        vec::from_fn(1000, |_| 1);
    }
}

fn main() {
}
