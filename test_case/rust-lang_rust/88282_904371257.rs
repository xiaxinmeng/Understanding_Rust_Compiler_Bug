plain
    Checking rustc-demangle v0.1.18
error[E0423]: expected value, found built-in attribute `start`
    --> library/alloc/src/collections/binary_heap.rs:1591:29
     |
1591 |         self.0.rebuild_tail(start);


error[E0599]: the method `rebuild_tail` exists for mutable reference `&'a mut BinaryHeap<T>`, but its trait bounds were not satisfied
     |
     |
1591 |         self.0.rebuild_tail(start);
     |                ^^^^^^^^^^^^ method cannot be called on `&'a mut BinaryHeap<T>` due to unsatisfied trait bounds
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `T: Ord`
Some errors have detailed explanations: E0423, E0599.
For more information about an error, try `rustc --explain E0423`.
error: could not compile `alloc` due to 2 previous errors
Build completed unsuccessfully in 0:01:13
