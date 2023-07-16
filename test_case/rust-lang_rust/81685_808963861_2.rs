
error[E0308]: mismatched types
 --> src/main.rs:6:45
  |
6 |     should_be_iterator(Some(()).into_iter().inspect(validate_filename));
  |                                             ^^^^^^^ one type is more general than the other
  |
  = note: expected type `FnOnce<(&(),)>`
             found type `FnOnce<(&(),)>`
