
error[E0308]: mismatched types
 --> src/main.rs:5:33
  |
5 |     let y: for<'a> fn(&'a ()) = x;
  |                                 ^ one type is more general than the other
  |
  = note: expected fn pointer `for<'a> fn(&'a ())`
             found fn pointer `fn(&())`

For more information about this error, try `rustc --explain E0308`.
