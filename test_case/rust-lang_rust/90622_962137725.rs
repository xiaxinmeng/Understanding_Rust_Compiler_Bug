
error[E0312]: lifetime of reference outlives lifetime of borrowed content...
 --> src/lib.rs:7:9
  |
7 |         self.data
  |         ^^^^^^^^^
  |
note: ...the reference is valid for the lifetime `'a` as defined here...
 --> src/lib.rs:5:6
  |
5 | impl<'a> Test<'a> {
  |      ^^
note: ...but the borrowed content is only valid for the anonymous lifetime defined here
 --> src/lib.rs:6:17
  |
6 |     fn get_data(&self) -> &'a [u8] {
  |                 ^^^^^
