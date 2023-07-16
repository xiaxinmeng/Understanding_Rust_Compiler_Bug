
error[[E0599]](https://doc.rust-lang.org/nightly/error-index.html#E0599): no method named `poll` found for opaque type `impl Future<Output = ()>` in the current scope
 --> src/lib.rs:8:9
  |
8 |     f().poll(cx);
  |         ^^^^ method not found in `impl Future<Output = ()>`
 --> /rustc/659e169d37990b9c730a59a96081f2ef7afbe8f1/library/core/src/future/future.rs:105:8
  |
  = note: the method is available for `Pin<&mut impl Future<Output = ()>>` here
  |
help: consider wrapping the receiver expression with the appropriate type
  |
8 |     Pin::new(&mut f()).poll(cx);
  |     +++++++++++++    +
