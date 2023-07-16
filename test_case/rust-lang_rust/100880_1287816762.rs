rust
use std::vec::Vec;

#[derive(Clone, Copy, Debug)]
struct Type;

fn main() {
    let mut array = [Type{}];
    let mut vect = vec![];
    vect.extend_from_slice(&array);
    println!("{:?}", vect);  
}
