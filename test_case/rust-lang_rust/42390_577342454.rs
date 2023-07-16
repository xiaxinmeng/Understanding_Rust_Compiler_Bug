
error[E0271]: type mismatch resolving `<S as IntoItem>::Item == ()`
  --> file.rs:17:5
   |
1  | fn foo<F, R>(f: F)
   |    ---
2  |     where F: FnOnce() -> R,
3  |           R: IntoItem<Item=()>
   |                       ------- required by this bound in `foo`
...
17 |     foo(|| S);
   |     ^^^ expected `()`, found `u32`
