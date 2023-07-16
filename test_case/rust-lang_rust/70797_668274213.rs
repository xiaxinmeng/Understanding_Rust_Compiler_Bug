rust
use std::collections::binary_heap::{BinaryHeap, PeekMut};

fn main() {
    let mut heap = BinaryHeap::from(vec![1, 2, 3]);
    match heap.peek_mut() {
        Some(p) => {
            PeekMut::pop(p);
            heap.push(4);
        }
        None => {}
    };
}
