
warning: non-binding `let` on a type that implements `Drop`
 --> src/lib.rs:7:9
  |
7 |         let _ = drop_flag_copy;
  |         ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: consider using an underscore-prefixed named binding or dropping explicitly with `std::mem::drop`
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#let_underscore_drop
