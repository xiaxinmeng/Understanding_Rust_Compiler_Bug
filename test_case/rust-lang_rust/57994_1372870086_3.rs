
error[[E0599]](https://doc.rust-lang.org/nightly/error-index.html#E0599): no method named `x` found for struct `S` in the current scope
  --> src/main.rs:10:7
   |
2  | struct S;
   | -------- method `x` not found for this struct
...
5  |     fn x(self: Pin<&mut Self>) {
   |        - the method is available for `Pin<&mut S>` here
...
10 |     S.x();
   |       ^ method not found in `S`
   |
help: consider wrapping the receiver expression with the appropriate type
   |
10 |     Pin::new(&mut S).x();
   |     +++++++++++++  +
