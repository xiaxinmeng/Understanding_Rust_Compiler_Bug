
error[E0072]: recursive type `T` has infinite size
 --> test.rs:1:1
  |
1 | struct T([T; 0]);
  | ^^^^^^^^^^^^^^^^^ recursive type has infinite size
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `T` representable

error: aborting due to previous error
