rust
pub struct Stuff {
    heap: BinaryHeap<T>,
}

impl Stuff {
    pub fn method(&mut self) {
        // temporarily violate heap invariant in place
        ...
        // then
        self.heap.rebuild();
    }
}
