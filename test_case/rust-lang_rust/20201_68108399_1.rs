 rust
extern crate repr;

use repr::Link;

fn main() {
    let link_error : (uint, uint) = Link.error().produce();
}
