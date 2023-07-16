
error[E0308]: mismatched types
 --> src/main.rs:5:33
  |
5 |     let y: for<'a> fn(&'a ()) = x;
  |                                 ^ expected concrete lifetime, found bound lifetime parameter 'a
  |
  = note: expected type `for<'a> fn(&'a ())`
             found type `fn(&'static ())`
