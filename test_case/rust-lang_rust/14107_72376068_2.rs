 rust
#![feature(core, test, io)]

extern crate test;

fn main() {
    let haystack = std::old_io::stdin().read_to_string().unwrap();

    let mut needle = "_";

    for _ in range(0, 10000000) {
        test::black_box(&mut needle);
        test::black_box(haystack.contains(needle));
    }
}
