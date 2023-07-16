
error[[E0271]](https://doc.rust-lang.org/nightly/error-index.html#E0271): type mismatch resolving `<S as IntoItem>::Item == ()`
  --> src/main.rs:19:5
   |
19 |     foo(|| S);
   |     ^^^ type mismatch resolving `<S as IntoItem>::Item == ()`
   |
note: expected this to be `()`
  --> src/main.rs:15:17
   |
15 |     type Item = u32;
   |                 ^^^
note: required by a bound in `foo`
  --> src/main.rs:4:17
   |
1  | fn foo<F, R>(f: F)
   |    --- required by a bound in this
...
4  |     R: IntoItem<Item = ()>,
   |                 ^^^^^^^^^ required by this bound in `foo`
