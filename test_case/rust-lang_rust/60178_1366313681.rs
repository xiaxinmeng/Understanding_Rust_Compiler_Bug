
error[E0072]: recursive type `MList` has infinite size
 --> src/lib.rs:1:1
  |
1 | enum MList { Cons(isize, MList), Nil }
  | ^^^^^^^^^^               ----- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
1 | enum MList { Cons(isize, Box<MList>), Nil }
  |                          ++++     +
