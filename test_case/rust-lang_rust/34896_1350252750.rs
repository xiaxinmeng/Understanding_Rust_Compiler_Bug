
error[E0308]: mismatched types
 --> f54.rs:5:13
  |
5 |         foo(y.clone());
  |         --- ^^^^^^^^^ expected struct `Foo`, found `&Foo`
  |         |
  |         arguments to this function are incorrect
  |
note: `Foo` does not implement `Clone`, so `&Foo` was cloned instead
 --> f54.rs:5:13
  |
5 |         foo(y.clone());
  |             ^
note: function defined here
 --> f54.rs:9:4
  |
9 | fn foo(x: Foo) {}
  |    ^^^ ------
