rust
// aux-build:add-trait-impl.rs

use std::collections::BinaryHeap;

#[macro_use]
extern crate add_trait_impl;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct PriorityQueueEntry<T> {
    value: T,
}

#[derive(PartialOrd, AddImpl)]
struct PriorityQueue<T>(BinaryHeap<PriorityQueueEntry<T>>);

fn main() {}
