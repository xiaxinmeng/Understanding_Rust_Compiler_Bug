diff
error[E0277]: the trait bound `T: Ord` is not satisfied
   --> src/main.rs:8:10
    |
8   | #[derive(serde::Serialize)]
    |          ^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `T`
9   | struct PriorityQueue<T>(BinaryHeap<PriorityQueueEntry<T>>);
    |                         --------------------------------- required by a bound introduced by this call
    |
note: required for `PriorityQueueEntry<T>` to implement `Ord`
   --> src/main.rs:3:37
    |
3   | #[derive(PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
    |                                     ^^^
    = note: required for `BinaryHeap<PriorityQueueEntry<T>>` to implement `Serialize`
note: required by a bound in `serialize_newtype_struct`
   --> /Users/maeda.takayuki/.cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.148/src/ser/mod.rs:904:12
    |
904 |         T: Serialize;
    |            ^^^^^^^^^ required by this bound in `serialize_newtype_struct`
    = note: this error originates in the derive macro `Ord` (in Nightly builds, run with -Z macro-backtrace for more info)
- help: consider further restricting type parameter `T`
-     |
- 8   | #[derive(serde::Serialize, T: std::cmp::Ord)]
-     |                          ++++++++++++++++++
+ help: consider restricting type parameter `T`
+   |
+ 9 | struct PriorityQueue<T: std::cmp::Ord>(BinaryHeap<PriorityQueueEntry<T>>);
+   |                       +++++++++++++++
