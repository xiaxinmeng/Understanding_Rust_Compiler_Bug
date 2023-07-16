 rust
#![feature(box_syntax)]

const LEN: usize = 1 << 15;

use std::thread::Builder;

fn main() {
    assert!(Builder::new().stack_size(LEN / 2).spawn(|| {
        let vec = <[_]>::into_vec(box ([0; LEN]);
        assert_eq!(vec.len(), LEN);
    }).unwrap().join().is_ok());
}
