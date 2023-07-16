rust
#![feature(unsized_locals)]

use std::ops::Index;

trait Trait {
    fn value(&self) -> usize;
}

struct DrStrange(Vec<i32>);

impl Index<Trait> for DrStrange {
    type Output = i32;
    fn index(&self, index: Trait) -> &i32 {
        &self.0[index.value()]
    }
}

impl Trait for usize {
    fn value(&self) -> usize { *self }
}

fn main() {
    let stephen = DrStrange(vec![1, 2, 3]);
    // ???? pnkfelix doesn't know offhand how to write
    // an input to fill into `stephen[...]`.
}
