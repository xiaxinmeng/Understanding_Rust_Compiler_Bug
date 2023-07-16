rust
// crate A
pub macro n() { println!("Hello world!") }
pub macro m() { n!() }

// crate B
extern crate A;
use A::m;

macro n() {}
macro println() {}
//^ These don't interfere with the expansion of `m!()` below.

fn main() {
    m!(); // prints "Hello world!"
}
