rust
#![feature(const_generics)]

use std::fmt;

struct Array<T>(T);

impl<T: fmt::Debug, const N: usize> fmt::Debug for Array<[T; N]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries((&self.0 as &[T]).iter()).finish()
    }
}

fn main() {
    println!("{:?}", Array([1, 2, 3]));
}
