 rust
use std::collections::VecDeque;

fn main() {
    let mut v = VecDeque::new();
    v.push_front(());
    println!("{}", v.len());
}
