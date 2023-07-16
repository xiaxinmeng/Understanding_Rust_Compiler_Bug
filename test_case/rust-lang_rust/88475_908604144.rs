
warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
  --> src/main.rs:20:20
   |
20 |     let y = points.into_iter();
   |                    ^^^^^^^^^
   |
note: the lint level is defined here
  --> src/main.rs:1:9
   |
1  | #![warn(array_into_iter)]
   |         ^^^^^^^^^^^^^^^
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
20 |     let y = points.iter();
   |                    ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
   |
20 |     let y = IntoIterator::into_iter(points);
   |             ++++++++++++++++++++++++      ~
