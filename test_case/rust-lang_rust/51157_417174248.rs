rust
warning: variable does not need to be mutable
 --> src/main.rs:5:9
  |
5 |     let mut ref_mut = refcell.borrow_mut();
  |         ----^^^^^^^
  |         |
  |         help: remove this `mut`
  |
  = note: #[warn(unused_mut)] on by default
