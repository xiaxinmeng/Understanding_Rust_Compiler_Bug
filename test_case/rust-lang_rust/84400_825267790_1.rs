
error[E0308]: mismatched types
 --> src/main.rs:7:33
  |
7 |     let _ = options.into_iter().filter(check::<&String>);
  |                                 ^^^^^^ one type is more general than the other
  |
  = note: expected type `FnOnce<(&String,)>`
             found type `FnOnce<(&String,)>`
