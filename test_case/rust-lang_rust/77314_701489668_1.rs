rust
fn main() {
    let a: &[u8] = &[];
    let b: &Vec<u8> = &vec![];
    a > b;
}

// ========== Comment out from this line ==========
use std::cmp::{Ordering, PartialOrd};

struct Foo();
impl PartialEq<[u8]> for Foo {
    fn eq(&self, other: &[u8]) -> bool {
        true
    }
}
impl PartialEq<Foo> for [u8] {
    fn eq(&self, other: &Foo) -> bool {
        true
    }
}
impl PartialOrd<[u8]> for Foo {
    fn partial_cmp(&self, other: &[u8]) -> Option<Ordering> {
        None
    }
}
impl PartialOrd<Foo> for [u8] {
    fn partial_cmp(&self, other: &Foo) -> Option<Ordering> {
        None
    }
}
