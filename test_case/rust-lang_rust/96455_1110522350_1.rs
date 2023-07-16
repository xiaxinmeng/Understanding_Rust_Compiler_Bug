console
error[E0716]: temporary value dropped while borrowed
  --> src/main.rs:18:12
   |
18 |     write!(MutexGuard(&mutex), "").lol();
   |     -------^^^^^^^^^^^^^^^^^^-----
   |     |      |
   |     |      creates a temporary which is freed while still in use
   |     temporary value is freed at the end of this statement
   |     borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value
