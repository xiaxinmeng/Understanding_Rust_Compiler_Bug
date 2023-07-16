rust
use std::fmt;

pub struct Wrapper(for<'b> fn(&'b ()));

impl fmt::Debug for Wrapper {
    fn fmt<'a>(&'a self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dyn_debug: &dyn fmt::Debug = &self.0 as &fn(&'static ());
        fmt::Debug::fmt(&dyn_debug, f)
    }
}

fn useful(_: &()) {}

fn main() {
    println!("{:?}", Wrapper(useful));
}
