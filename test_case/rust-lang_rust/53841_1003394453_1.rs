`
error[E0618]: expected function, found `bool`
 --> src/main.rs:7:9
  |
5 |     let a: bool = false;
  |         - `a` has type `bool`
6 |     if a {
7 |         a();
  |         ^--
  |         |
  |         call expression requires function
  