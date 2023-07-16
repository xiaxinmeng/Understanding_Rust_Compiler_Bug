
error[E0369]: binary operation `==` cannot be applied to type `MyHashSet<T>`
  --> test.rs:21:16
   |
21 |         self.0 == other.0
   |         ------ ^^ ------- MyHashSet<T>
   |         |
   |         MyHashSet<T>
   |
help: consider further restricting this bound
   |
19 | impl<T> PartialEq for CustomSet<T> where T: PartialEq + std::cmp::PartialEq {
   |                                                       ^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
