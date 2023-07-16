rust
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
struct PriorityQueueEntry<T> {
    value: T,
}

#[derive(serde::Serialize)]
struct PriorityQueue<T>(BinaryHeap<PriorityQueueEntry<T>>);
