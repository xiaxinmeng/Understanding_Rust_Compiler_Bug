rust
error: borrow expressions cannot be annotated with lifetimes
 --> src/main.rs:5:9
  |
5 |     foo(&'a bar);
  |         ^--^^^^
  |          |
  |          annotated with lifetime here
  |          help: remove the lifetime annotation
