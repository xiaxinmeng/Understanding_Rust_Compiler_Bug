rust
use std::collections::VecDeque;

fn main() {
    let v = vec![(); 100];
    let queue = VecDeque::from(v);
    println!("{:?}", queue);
}
