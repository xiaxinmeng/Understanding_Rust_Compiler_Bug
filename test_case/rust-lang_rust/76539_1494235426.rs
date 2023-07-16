
error[[E0369]](https://doc.rust-lang.org/stable/error_codes/E0369.html): cannot add `usize` to `&T`
 --> src/lib.rs:6:26
  |
6 |         counter = number + counter;
  |                   ------ ^ ------- usize
  |                   |
  |                   &T
  |
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
  |
3 | fn sum<T: Add<usize, Output = usize>>(list: &[T]) -> usize where &T: Add<usize, Output = usize> {
  |                                                            ++++++++++++++++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `playground` due to previous error
