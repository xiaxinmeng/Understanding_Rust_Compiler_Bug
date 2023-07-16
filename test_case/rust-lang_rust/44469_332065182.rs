rust
use std::fmt::Debug;

trait DescriptiveSpec<'r> {}

impl<'r, T> DescriptiveSpec<'r> for &'r T {}

fn from_spec<'r, T: DescriptiveSpec<'r>>(spec: &'r T)  {}

fn matching_contains<'s, T: 's, I>(a: &mut &'s I) where &'s I: Debug {
    from_spec(a);
}

fn main() {}
