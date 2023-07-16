
error[E0061]: this function takes 3 parameters but 4 parameters were supplied
 --> src/main.rs:6:5
  |
2 | fn foo(_: usize, _: usize, _: usize) {}
  | ------------------------------------ defined here
...
6 |     T::foo(1, 3, 4, 5);
  |     ^^^^^^^^^^^^^^^^^^ expected 3 parameters
