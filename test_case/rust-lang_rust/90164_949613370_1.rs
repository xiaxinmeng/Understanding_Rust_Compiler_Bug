
error[E0277]: `T` cannot be unpinned
 --> src/lib.rs:5:13
  |
5 |     copy(r, w);
  |     ----    ^ the trait `Unpin` is not implemented for `T`
  |     |
  |     required by a bound introduced by this call
  |
  = note: consider using `Box::pin`
note: required by a bound in `copy`
 --> src/lib.rs:1:12
  |
1 | fn copy<R: Unpin, W>(_: R, _: W) {}
  |            ^^^^^ required by this bound in `copy`
help: consider restricting type parameter `T`
  |
3 | fn f<T: std::marker::Unpin>(r: T) {
  |       ++++++++++++++++++++
