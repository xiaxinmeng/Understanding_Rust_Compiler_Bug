
warning: value assigned to `foo` is never read
 --> src/main.rs:2:10
  |
2 |     let mut foo = String::new();
  |             ^^^
  |
  = note: #[warn(unused_assignments)] on by default
  = help: maybe it is overwritten before being read?
