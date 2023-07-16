 rust
...
struct Something {
    heap: BinaryHeap<Panicker>
}

impl Drop for Something {
    fn drop(&mut self) {
        ... self.heap.iter().next() ...
    }
}

let mut heap = Something { heap: BinaryHeap::new() };
heap.push(Panicker { ... });
