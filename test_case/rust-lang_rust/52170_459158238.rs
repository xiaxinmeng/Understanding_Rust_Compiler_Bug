
error[E0507]: cannot move out of borrowed content
 --> src/main.rs:4:53
  |
4 | let mut f: Box<FnMut(&(i32, Vec<i32>))> = Box::new(|(_, mut v)| {
  |                                                     ^^^^-----^
  |                                                     |   |
  |                                                     |   data moved here
  |                                                     cannot move out of borrowed content
  |
note: move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
 --> src/main.rs:4:57
  |
4 | let mut f: Box<FnMut(&(i32, Vec<i32>))> = Box::new(|(_, mut v)| {
  |                                                         ^^^^^
