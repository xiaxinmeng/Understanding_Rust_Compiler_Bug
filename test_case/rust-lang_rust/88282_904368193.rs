plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.18
error[E0428]: the name `RebuildTail` is defined multiple times
     |
     |
1570 | struct RebuildTail<T>(&mut BinaryHeap<T>);
     | ------------------------------------------ previous definition of the type `RebuildTail` here
...
1597 | struct RebuildTail<'a,T>(&'a mut BinaryHeap<T>);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `RebuildTail` redefined here
     |
     = note: `RebuildTail` must be defined only once in the type namespace of this module
error[E0423]: expected value, found built-in attribute `start`
    --> library/alloc/src/collections/binary_heap.rs:1601:29
     |
     |
1601 |         self.0.rebuild_tail(start);


error[E0107]: this struct takes 0 lifetime arguments but 1 lifetime argument was supplied
     |
     |
1598 | impl<'a,T> Drop for RebuildTail<'a,T> {
     |                     ^^^^^^^^^^^ -- help: remove this lifetime argument
     |                     expected 0 lifetime arguments
     |
note: struct defined here, with 0 lifetime parameters
    --> library/alloc/src/collections/binary_heap.rs:1570:8
    --> library/alloc/src/collections/binary_heap.rs:1570:8
     |
1570 | struct RebuildTail<T>(&mut BinaryHeap<T>);

error[E0106]: missing lifetime specifier
    --> library/alloc/src/collections/binary_heap.rs:1570:23
     |
     |
1570 | struct RebuildTail<T>(&mut BinaryHeap<T>);
     |                       ^ expected named lifetime parameter
     |
help: consider introducing a named lifetime parameter
     |
1570 | struct RebuildTail<'a, T>(&'a mut BinaryHeap<T>);
     |                    ^^^    ^^^
Some errors have detailed explanations: E0106, E0107, E0423, E0428.
For more information about an error, try `rustc --explain E0106`.
error: could not compile `alloc` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
