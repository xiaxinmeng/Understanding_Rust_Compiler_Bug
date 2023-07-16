
error[E0277]: the size for values of type `T` cannot be known at compilation time
 --> src/main.rs:9:22
  |
4 | const fn batch_size<T: Sized>() -> usize {
  |          ---------- - required by this bound in `batch_size`
...
8 | fn x<T: Sized>() {
  |      -- help: consider further restricting this bound: `T: std::marker::Sized +`
9 |     [1; batch_size::<T>()];
  |                      ^ doesn't have a size known at compile-time
  |
