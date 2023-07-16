rust
use std::convert::TryInto;

fn main() {
    let x: [u8; 3] = vec!(1, 2, 3)[..].try_into().unwrap(); // sized array
    let y: &[u8] = &vec!(1, 2, 3)[..]; // `u8` slice
}
