
error[E0596]: cannot borrow `*self` as mutable, as it is behind a `&` reference
 --> src/main.rs:3:9
  |
3 |         self.bar()
  |         ^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
2 |     fn foo(&mut self) {
  |            ~~~~~~~~~
