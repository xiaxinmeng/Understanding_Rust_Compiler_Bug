rust
use std::panic::Location;

macro_rules! f {
    () => { Location::caller() }
}

#[inline(always)]
fn g() -> &'static Location<'static> {
    f!()
}

fn main() {
    println!("{:?}", g());
}
