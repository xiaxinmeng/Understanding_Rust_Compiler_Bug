rust
use std::vec::Vec;

#[derive(Clone, Copy, Debug)]
struct Type;

fn main() {
    let mut array = [Type{}];
    let mut vect = Vec::from(&array);
    println!("{:?}", vect);
    
}
