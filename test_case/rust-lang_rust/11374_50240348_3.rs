
use std::vec;

use m1;

#[main]
fn main() {
    let c = m1::for_stdin();
    let mut v = vec::Vec::from_elem(10, 0u8);
    c.read_to(v.as_mut_slice());
}
