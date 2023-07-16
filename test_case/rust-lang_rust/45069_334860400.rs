rust
error[E0593]: closure takes multiple arguments, but a tuple argument is required
 --> test.rs:5:40
  |
5 |     let it = v.into_iter().enumerate().map(|i, x| i);
  |                                        ^^^ ------ consider changing to `|(i, x)|`
  |                                        |
  |                                        expected closure that takes 1 argument, a 2-tuple
