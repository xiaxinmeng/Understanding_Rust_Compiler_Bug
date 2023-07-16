console
error[E0308]: mismatched types
 --> src/main.rs:5:5
  |
5 |     my_function(closure_consumer);
  |     ^^^^^^^^^^^ one type is more general than the other
  |
  = note: expected type `FnOnce<(&mut i32, &mut i32)>`
             found type `for<'r> FnOnce<(&'r mut i32, &mut i32)>`
