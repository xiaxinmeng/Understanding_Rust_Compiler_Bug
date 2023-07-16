
error[E0631]: type mismatch in function arguments
 --> ./ice.rs:7:47
  |
1 | fn score(_: (usize, &str)) -> usize {
  | ----------------------------------- found signature of `for<'r> fn((usize, &'r str)) -> _`
...
7 |     let result = names.iter().enumerate().map(score);
  |                                               ^^^^^ expected signature of `fn((usize, &&str)) -> _`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0631`.
