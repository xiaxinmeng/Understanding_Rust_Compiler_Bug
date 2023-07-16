 rust
use std::cmp::PartialEq;

struct Input<'a> {
    bytes: &'a [u8],
}

impl <'a> PartialEq for Input<'a> {
    #[inline]
    fn eq(&self, _other: &Input<'a>) -> bool {
        panic!()
    }
}

struct Input2<'a> {
    i: Input<'a>
}

impl<'a, 'b> PartialEq<Input2<'b>> for Input2<'a> {
    #[inline]
    fn eq(&self, other: &Input2<'b>) -> bool {
        self.i == other.i // error here
    }
}

fn main() {}
