 Rust
use std::hash::{hash, SipHasher};

#[derive(Hash)]
enum E {
    A = 1,
    B,
}

fn main() {
    let a = hash::<_,   SipHasher>(&E::A);
    let b = hash::<_,   SipHasher>(&E::B);

    println!("{}, {}, {}", a, b, a == b);
}
