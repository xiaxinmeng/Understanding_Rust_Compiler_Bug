text
error[E0391]: cycle detected when computing the supertraits of `Stream`
 --> src/main.rs:5:1
  |
5 | trait Stream<T>: Future<Option<(T, Box<Stream<T>>)>> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: ...which again requires computing the supertraits of `Stream`, completing the cycle

