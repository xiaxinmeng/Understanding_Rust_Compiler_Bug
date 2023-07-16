
error[[E0391]](https://doc.rust-lang.org/nightly/error-index.html#E0391): cycle detected when computing the bounds for type parameter `Self`
 --> src/lib.rs:6:40
  |
6 |         Self: for<'s> Gen<'s, Output = Self::Output>;
  |                                        ^^^^^^^^^^^^
  |
  = note: ...which immediately requires computing the bounds for type parameter `Self` again
note: cycle used when computing function signature of `Gen::gen`
 --> src/lib.rs:4:22
  |
4 |     fn gen(&self) -> Self::Output
  |                      ^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0391`.
