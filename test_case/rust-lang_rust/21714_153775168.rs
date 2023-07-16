
use std::hash::Hasher;

#[derive(Hash)]
enum E {
    A = 1,
    B,
}

fn main() {
    let mut hasher = SipHasher::new();

    hasher.write_i32(E::A as i32);
    let a = hasher.finish();

    let mut hasher = SipHasher::new();
    hasher.write_i32(E::B as i32);
    let b = hasher.finish();

    println!("{:?}, {:?}, {:?}", a, b, a == b);
}
