
use std::fmt;

trait MyTrait { }

impl<T: MyTrait> fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

fn main() {}

