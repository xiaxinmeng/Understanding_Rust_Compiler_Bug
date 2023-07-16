 rust
use std::ops::Index;

struct S;

impl Index<uint, [u8]> for S {
    fn index<'a>(&'a self, _: &uint) -> &'a [u8] {
        b"a"
    }
}

fn main() {
    &S[0];
}
