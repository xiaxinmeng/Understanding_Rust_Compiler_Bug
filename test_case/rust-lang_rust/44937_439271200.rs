
use std::num::Wrapping;

fn main() {
    let mut x = Wrapping(3);
    
    x *= 4;

    println!("{}", x);
}
