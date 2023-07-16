
error[E0271]: type mismatch resolving `<S as IntoItem>::Item == ()`
  --> file.rs:17:5
   |
1  | fn foo<F, R>(f: F)
   |    ---
2  |     where F: FnOnce() -> R,
3  |           R: IntoItem<Item=()>
   |                       ------- required by this bound in `foo`
...
12 | impl IntoItem for S {
   | ------------------- in this `impl`
13 |     type Item = u32;
   |     --------------- found because of this bound for `S`
...
17 |     foo(|| S);
   |     ^^^ expected `()`, found `u32`
