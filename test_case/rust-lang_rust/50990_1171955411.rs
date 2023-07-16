rust
use std::fmt::Debug;

fn xxx<A>(x: impl Debug) {
    println!("Done! {:?}", x)
}

fn main() {
    xxx::<()>(123u8);
}
