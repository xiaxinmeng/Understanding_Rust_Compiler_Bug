rust
#![feature(unboxed_closures)]

trait Lt<'_> {
    type T = ();
}
impl<'f> Lt<'unboxed_closures> for () {
    type T = ();
}

fn main() {
    let v:<() as Lt<'_>>::T = ();
}
