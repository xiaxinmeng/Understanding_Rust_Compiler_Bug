rust
// the writing is on the wall, let's not look at the entire wall
#![recursion_limit="10"]

struct Encoder {}

trait CanEncode { fn write(self); }

impl Encoder {
    fn write<E: CanEncode>(&mut self, x: E) { x.write() }
}

impl<'a, T> CanEncode for &'a [T] where
    &'a T: CanEncode
{
    fn write(self) {}
}

fn main() {
    let mut wtr = Encoder {};
    wtr.write::<&[_]>(42).unwrap();
}
