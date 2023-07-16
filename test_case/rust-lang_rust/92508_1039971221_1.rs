
error[[E0716]](https://doc.rust-lang.org/stable/error-index.html#E0716): temporary value dropped while borrowed
  [--> src/main.rs:22:12
](https://play.rust-lang.org/#)   |
22 |     a[&mut ()] = ();
   |            ^^      - temporary value is freed at the end of this statement
   |            |
   |            creates a temporary which is freed while still in use
23 |     a[&mut ()] = ();
   |     - borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value
