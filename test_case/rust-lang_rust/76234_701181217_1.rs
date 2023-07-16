rust
pub struct Unsorted<T>(BinaryHeap<T>);
pub struct UnsortedMut<'a, T>(&'a mut BinaryHeap<T>);
