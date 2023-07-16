
#[macro_use]
extern crate bug;

use bug::Trait;

struct Dummy;

fn main() {
    Dummy.method();
}
