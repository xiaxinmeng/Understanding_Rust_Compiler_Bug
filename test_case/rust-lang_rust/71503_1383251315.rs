rust
#![feature(binary_heap_retain)]

use std::collections::BinaryHeap;
use std::panic::{self, AssertUnwindSafe};

fn main() {
    let mut heap = BinaryHeap::from_iter([3, 1, 2]);
    println!("heap={:?}", heap);
    panic::catch_unwind(AssertUnwindSafe(|| {
        heap.retain(|e| {
            if *e == 1 {
                panic!();
            }
            false
        });
    }));
    println!("heap={:?}", heap);
    println!("sorted={:?}", heap.into_sorted_vec());
}
