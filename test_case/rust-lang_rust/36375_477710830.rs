rust
use std::fmt::Debug;

fn main() {
    println!("{:?}", frob());
}

fn frob() -> impl Debug {
    unimplemented!()
}
