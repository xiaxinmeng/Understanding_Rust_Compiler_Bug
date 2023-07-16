 rust
extern crate test;

use test::{Bencher, black_box};
use std::io::BufReader;

#[bench]
fn read_to_end(b: &mut Bencher) {
    let bytes = Vec::from_elem(100, 10u8);
    b.iter(|| {
        let mut reader = BufReader::new(bytes.as_slice());
        black_box(reader.read_to_end())
    })
}
