
ff497332 src/libcollections/binary_heap.rs       (Alex Crichton              2015-10-22 16:28:45 -0700 1282) impl<T: Ord> From<Vec<T>> for BinaryHeap<T> {
e30480ca src/liballoc/collections/binary_heap.rs (Observer42                 2019-08-12 18:07:14 +0800 1283)     /// Converts a `Vec<T>` into a `BinaryHeap<T>`.
fa7a40cc src/liballoc/collections/binary_heap.rs (Observer42                 2019-08-12 16:07:13 +0800 1284)     ///
e30480ca src/liballoc/collections/binary_heap.rs (Observer42                 2019-08-12 18:07:14 +0800 1285)     /// This conversion happens in-place, and has `O(n)` time complex
ity.
ff497332 src/libcollections/binary_heap.rs       (Alex Crichton              2015-10-22 16:28:45 -0700 1286)     fn from(vec: Vec<T>) -> BinaryHeap<T> {
