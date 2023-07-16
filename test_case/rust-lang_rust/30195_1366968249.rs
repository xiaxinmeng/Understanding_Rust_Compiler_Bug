
error[E0308]: mismatched types
  --> src/main.rs:11:9
   |
11 |     foo(S);
   |     --- ^ expected struct `K`, found struct `S`
   |     |
   |     arguments to this function are incorrect
   |
note: function defined here
  --> src/main.rs:8:4
   |
8  | fn foo(_: K) {}
   |    ^^^ ----
help: call `Into::into` on this expression to convert `S` into `K`
   |
11 |     foo(S.into());
   |          +++++++
