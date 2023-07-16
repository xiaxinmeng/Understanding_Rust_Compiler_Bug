rust
use collections::vec;
use heap::Heap;

pub type Vec<T, A: Alloc = Heap> = vec::Vec<T, A>;
