rust
use playground::Trait;

struct Struct;

impl Trait for Struct {
    fn method() {
        const _: () = {
            extern crate playground;
        };
    }
}

fn main() {}
