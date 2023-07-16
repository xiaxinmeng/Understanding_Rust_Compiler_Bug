
error[[E0599]](https://doc.rust-lang.org/nightly/error-index.html#E0599): no method named `poll` found for type parameter `T` in the current scope
  --> src/lib.rs:10:16
   |
5  | impl<T> Wrapper<T>
   |      - method `poll` not found for this type parameter
...
10 |         self.0.poll(todo!());
   |                ^^^^ method not found in `T`
  --> /rustc/61a415be590113b4935464ef0aaf3b4e7713a077/library/core/src/future/future.rs:105:8
   |
   = note: the method is available for `Pin<&mut T>` here
   |
help: consider wrapping the receiver expression with the appropriate type
   |
10 |         Pin::new(&mut self.0).poll(todo!());
   |         +++++++++++++       +
