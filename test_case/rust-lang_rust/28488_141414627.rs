 rust
use std::collections::VecDeque;

fn main() {
    let mut v = VecDeque::new();
    v.push_front(());                                                                                                         
    assert_eq!(v.len(), 1);   
}
