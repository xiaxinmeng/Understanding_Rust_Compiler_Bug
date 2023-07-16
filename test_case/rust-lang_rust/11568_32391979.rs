 rust
use std::num;

fn main() {
    let a = 2u & 1 == 0;
    let b = 2u.is_even();

    println!("{}", a);
    println!("{}", b);
}
