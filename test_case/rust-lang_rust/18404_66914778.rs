 rust
extern crate test;

use test::{Bencher, black_box};

#[bench]
pub fn to_string(b: &mut Bencher) {
    b.iter(|| {
        black_box("".to_string());
    });
}

#[bench]
pub fn from_str(b: &mut Bencher) {
    b.iter(|| {
        black_box(String::from_str(""));
    });
}
