
error[E0271]: type mismatch resolving `<X as Foo>::Item == f64`
  --> src/main.rs:16:5
   |
13 | fn foo(_: impl Foo<Item = f64>) {}
   |    ---             ---------- required by this bound in `foo`
...
16 |     foo(X);
   |     ^^^ expected i32, found f64
