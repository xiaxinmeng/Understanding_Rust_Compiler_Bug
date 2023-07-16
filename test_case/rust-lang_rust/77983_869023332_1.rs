
C:\Users\Administrator\CLionProjects\xx>cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\xx.exe`
Hello, world!

C:\Users\Administrator\CLionProjects\xx>cargo clippy
warning: a `const` item should never be interior mutable
 --> src\main.rs:4:1
  |
4 | pub const A: AtomicBool = AtomicBool::new(true);
  | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  | |
  | make this a static item (maybe with lazy_static)
  |
  = note: `#[warn(clippy::declare_interior_mutable_const)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#declare_interior_mutable_const

warning: a `const` item with interior mutability should not be borrowed
 --> src\main.rs:8:5
  |
8 |     A.store(false, SeqCst);
  |     ^
  |
  = note: `#[warn(clippy::borrow_interior_mutable_const)]` on by default
  = help: assign this const to a local or static variable, and use the variable here
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#borrow_interior_mutable_const

warning: 2 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
