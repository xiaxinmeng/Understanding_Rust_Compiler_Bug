rust
use std::mem;

fn main() {
    enum E { V = 42 as isize }
    let e = E::V;
    println!("{}", mem::size_of::<E>());
    println!("{}", e as isize);
}
