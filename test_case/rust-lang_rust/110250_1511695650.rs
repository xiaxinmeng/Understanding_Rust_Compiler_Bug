
error[[E0507]](https://doc.rust-lang.org/stable/error_codes/E0507.html): cannot move out of `*x` which is behind a shared reference
 --> src/main.rs:9:5
  |
9 |     (*x).a();
  |     ^^^^ --- `*x` moved due to this method call
  |     |
  |     move occurs because `*x` has type `A`, which does not implement the `Copy` trait
  |
note: `A::a` takes ownership of the receiver `self`, which moves `*x`
 --> src/main.rs:4:10
  |
4 |     fn a(self) {}
  |          ^^^^
