rust
use std::ops::AddAssign;
struct Foo;

impl AddAssign<()> for Foo {
    fn add_assign(&mut self, _: ()) {}
}

impl AddAssign<()> for &mut Foo {
    fn add_assign(&mut self, _: ()) {}
}

fn main() {
    (&mut Foo) += ();
}
