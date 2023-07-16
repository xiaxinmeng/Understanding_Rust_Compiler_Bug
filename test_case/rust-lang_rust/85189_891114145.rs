
warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
 --> src/main.rs:2:23
  |
2 |     let _ = [1, 2, 3].into_iter();
  |                       ^^^^^^^^^
  |
  = note: `#[warn(array_into_iter)]` on by default
  = warning: this changes meaning in Rust 2021
  = note: for more information, see issue #66145 <https://github.com/rust-lang/rust/issues/66145>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
  |
2 |     let _ = [1, 2, 3].iter();
  |                       ^^^^
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
  |
2 |     let _ = IntoIterator::into_iter([1, 2, 3]);
  |             ^^^^^^^^^^^^^^^^^^^^^^^^         ^
