rust
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:44
  |
3 | pub fn readv_at(bufs: &mut [&mut [u8]]) -> NestedList {
  |                       ----------------     ^^^^^^^^^^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say which one of `bufs`'s 2 lifetimes it is borrowed from
help: consider introducing a named lifetime parameter
  |
3 | pub fn readv_at<'a>(bufs: &'a mut [&'a mut [u8]]) -> NestedList<'a> {
  |                ++++        ++       ++                         ++++

error[E0308]: mismatched types
 --> src/main.rs:4:5
  |
3 | pub fn readv_at(bufs: &mut [&mut [u8]]) -> NestedList {
  |                                            ---------- expected `NestedList<'_>` because of return type
4 |     bufs
  |     ^^^^ expected `NestedList<'_>`, found `&mut [&mut [u8]]`
  |
help: try wrapping the expression in `NestedList`
  |
4 |     NestedList(bufs)
  |     +++++++++++    +
