Rust
use std::iter::{Fuse, Cloned};
use std::slice::Iter;

fn main() {
    let f = 0 as *mut <Fuse<Cloned<Iter<u8>>> as Iterator>::Item;
}
