rust
pub struct Sorted<T>(BinaryHeap<T>);
pub struct SortedMut<'a, T>(&'a mut BinaryHeap<T>);

// current into_iter_sorted
impl<T> IntoIterator for Sorted<T> {
    type Item = T;
}

// equivalent to SortedMut<'a, T> as IntoIterator
impl<'a, T> IntoIterator for &'a mut Sorted<T> {
    type Item = T;
}

impl<'a, T> IntoIterator for SortedMut<'a, T> {
    type Item = T;
}

impl<'a, T> SortedMut<'a, BinaryHeap<T>> {
    // current drain_sorted
    pub fn drain(self) -> SortedMutDrain<'a, T>;
}

impl<T> BinaryHeap<T> {
    pub fn into_sorted(self) -> Sorted<T>;
    pub fn sorted_mut(&mut self) -> SortedMut<T>;
}
