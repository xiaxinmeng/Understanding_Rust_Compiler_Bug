console
error: implementation of `IsOne` is not general enough
  --> src/main.rs:13:5
   |
8  | pub trait IsOne<T> {}
   | --------------------- trait `IsOne` defined here
...
13 |     assert_send(async {
   |     ^^^^^^^^^^^ implementation of `IsOne` is not general enough
   |
   = note: `One<Box<(dyn Send + Sync + '0)>>` must implement `IsOne<Box<(dyn Send + Sync + '1)>>`, for any two lifetimes `'0` and `'1`...
   = note: ...but `IsOne<Box<dyn Send + Sync>>` is actually implemented for the type `One<Box<dyn Send + Sync>>`
