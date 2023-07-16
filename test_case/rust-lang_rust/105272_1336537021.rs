
warning: value assigned to `placeholder` is never read
 --> src/main.rs:8:13
  |
8 |     let mut placeholder = String::new();
  |             ^^^^^^^^^^^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default
