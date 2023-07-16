
error[E###]: array initializers may not have length-dependent side-effects
 --> src/main.rs:2:23
  |
2 |     let mut strings = [String::with_capacity(32); 16];
  |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `String` is not `Copy` nor is `String::with_capacity(32)` a const-expression
  |
  = note: the element type must either be copyable or the initializer expression must be const so as to not have length-dependent side-effects
