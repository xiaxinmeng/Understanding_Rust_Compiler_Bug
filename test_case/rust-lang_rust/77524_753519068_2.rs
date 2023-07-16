
error[E0061]: this function takes 1 argument but 2 arguments were supplied
 --> src/lib.rs:6:5
  |
6 |     foo(1, 2);
  |     ^^^ -  - supplied 2 arguments
  |     |
  |     expected 1 argument
  |
note: function defined here
 --> src/lib.rs:1:4
  |
1 | fn foo(x: usize) {
  |    ^^^ --------
