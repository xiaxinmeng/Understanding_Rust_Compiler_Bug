
warning: using `clone` on a double-reference; this will copy the reference instead of cloning the inner type
 --> src/main.rs:7:5
  |
7 |     b.clone().fetch_add(1, Ordering::SeqCst);
  |     ^^^^^^^^^ help: try dereferencing it `(*b).clone()`
  |
  = note: #[warn(clone_double_ref)] on by default
  = help: for further information visit https://github.com/Manishearth/rust-clippy/wiki#clone_double_ref
