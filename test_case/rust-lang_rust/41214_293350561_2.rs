rust
error[E0072]: recursive type `A` has infinite size
 --> file2.rs:1:1
  |
1 |   struct
  |  _^ starting here...
2 | | A
  | |_^ ...ending here: recursive type has infinite size
