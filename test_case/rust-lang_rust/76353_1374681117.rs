
error[E0308]: mismatched types
 --> src/main.rs:9:7
  |
9 |     f(inner);
  |     - ^^^^^ one type is more general than the other
  |     |
  |     arguments to this function are incorrect
  |
  = note: expected fn pointer `for<'a, 'b, 'c> fn(&'a str, &'b S<'c>)`
             found fn pointer `fn(_, _)`
note: function defined here
 --> src/main.rs:3:4
  |
3 | fn f(inner: fn(&str, &S)) {
  |    ^ -------------------
help: consider removing the ``
  |
9 |     f(inner);
  |
