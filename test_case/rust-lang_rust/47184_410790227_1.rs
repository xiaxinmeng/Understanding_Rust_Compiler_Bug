
error[E0597]: borrowed value does not live long enough
 --> src/main.rs:4:43
  |
4 |     let vec: Vec<&'static String> = vec![&String::new()];
  |                                           ^^^^^^^^^^^^^ - temporary value only lives until here
  |                                           |
  |                                           temporary value does not live long enough
  |
  = note: borrowed value must be valid for the static lifetime...
  = note: consider using a `let` binding to increase its lifetime
