
   Compiling playground v0.0.1 (/playground)
error[E0507]: cannot move out of `self.inner.inner` which is behind a shared reference
  --> src/main.rs:26:18
   |
26 |         for _ in self.inner.inner {}
   |                  ^^^^^^^^^^^^^^^^
   |                  |
   |                  move occurs because `self.inner.inner` has type `Vec<&<D as Data<'_>>::Item>`, which does not implement the `Copy` trait
   |                  help: consider iterating over a slice of the `Vec<_>`'s content: `&self.inner.inner`

For more information about this error, try `rustc --explain E0507`.
error: could not compile `playground` due to previous error
