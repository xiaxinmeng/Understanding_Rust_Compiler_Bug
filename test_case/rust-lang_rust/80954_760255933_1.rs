
warning: value assigned to `n` is never read
 --> a.rs:5:30
  |
5 |         { println!("{}", n); n += 1; false } => {}
  |                              ^
  |
  = note: `#[warn(unused_assignments)]` on by default
  = help: maybe it is overwritten before being read?
