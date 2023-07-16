
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/lib.rs:7:25
  |
6 |     for reference in vec![1, 2, 3] {
  |         --------- expected due to the type of this binding
7 |         if /*let*/ Some(reference) = cache.data.get(*key) {
  |                         ^^^^^^^^^ expected integer, found `&i32`
  |
help: consider dereferencing the borrow
  |
7 |         if /*let*/ Some(*reference) = cache.data.get(*key) {
  |                         +

error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/lib.rs:7:20
  |
7 |         if /*let*/ Some(reference) = cache.data.get(*key) {
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`
  |
help: consider adding `let`
  |
7 |         if /*let*/ let Some(reference) = cache.data.get(*key) {
  |                    +++
