
error[E0600]: cannot apply unary operator `-` to type `T`
 --> src/lib.rs:2:9
  |
2 |     |a| -a
  |         ^^ cannot apply unary operator `-`
  |
help: consider restricting type parameter `T`
  |
1 | fn closure_unary<T: std::ops::Neg<Output = T>>() -> impl Fn(T) -> T {
  |                   +++++++++++++++++++++++++++

error[E0369]: cannot add `T` to `T`
 --> src/lib.rs:6:10
  |
6 |     |a| a+a
  |         -^- T
  |         |
  |         T
  |
help: consider restricting type parameter `T`
  |
5 | fn closure_binary<T: std::ops::Add<Output = T>>() -> impl Fn(T) -> T {
  |                    +++++++++++++++++++++++++++

Some errors have detailed explanations: E0369, E0600.
For more information about an error, try `rustc --explain E0369`.
