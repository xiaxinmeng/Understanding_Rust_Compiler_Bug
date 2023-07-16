rust
use std::iter::TrustedLen;

struct S(usize);

impl Clone for S {
    fn clone(&self) -> Self {
        S(0)
    }
}

impl Iterator for S {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 > 0 {
            self.0 -= 1;
            Some(())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0, Some(self.0))
    }
}

unsafe impl TrustedLen for S {}

fn main() {
    for x in S(4).cycle() {
        println!("{:?}", x);
    }
}
