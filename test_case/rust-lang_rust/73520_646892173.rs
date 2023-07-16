
error: `impl` item signature doesn't match `trait` item signature
 --> src/lib.rs:6:5
  |
2 |     fn make_it(&self) -> T;
  |     ----------------------- expected `fn(&()) -> &u8`
...
6 |     fn make_it(&self) -> &u8 {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&()) -> &u8`
  |
  = note: expected `fn(&()) -> &u8`
             found `fn(&()) -> &u8`
help: the lifetime requirements from the `impl` do not correspond to the requirements in the `trait`
 --> src/lib.rs:2:26
  |
2 |     fn make_it(&self) -> T;
  |                          ^ consider borrowing this type parameter in the trait
