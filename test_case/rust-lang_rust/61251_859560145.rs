
error[E0061]: this function takes 3 arguments but 4 arguments were supplied
 --> src/main.rs:6:5
  |
6 |     T::foo(1, 3, 4, 5);
  |     ^^^^^^ -  -  -  - supplied 4 arguments
  |     |
  |     expected 3 arguments
  |
note: associated function defined here
 --> src/main.rs:2:8
  |
2 |     fn foo(_: usize, _: usize, _: usize) {}
  |        ^^^ --------  --------  --------
