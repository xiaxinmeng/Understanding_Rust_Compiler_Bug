
% rustc +stable --version
rustc 1.58.1 (db9d1b20b 2022-01-20)
% rustc +nightly --version
rustc 1.60.0-nightly (1e12aef3f 2022-02-13)
% rustc +stable demo-change.rs
% rustc +nightly demo-change.rs
16-33-45 rust-92508/objdir-dbgopt (git:fix-64130-refine-scopes) % ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc demo-change.rs
error[E0716]: temporary value dropped while borrowed
  --> demo-change.rs:22:12
   |
22 |     a[&mut ()] = ();
   |     -------^^------
   |     |      | |
   |     |      | temporary value is freed at the end of this statement
   |     |      creates a temporary which is freed while still in use
   |     borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value

error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
