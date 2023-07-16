
error: a `const` item should never be interior mutable
 --> src/main.rs:3:1
  |
3 | const COUNTER: AtomicUsize = AtomicUsize::new(0);
  | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  | |
  | make this a static item (maybe with lazy_static)
  |
  = note: `#[deny(clippy::declare_interior_mutable_const)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#declare_interior_mutable_const

error: a `const` item with interior mutability should not be borrowed
 --> src/main.rs:6:5
  |
6 |     COUNTER.fetch_add(1, Ordering::SeqCst);
  |     ^^^^^^^
  |
  = note: `#[deny(clippy::borrow_interior_mutable_const)]` on by default
  = help: assign this const to a local or static variable, and use the variable here
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#borrow_interior_mutable_const
