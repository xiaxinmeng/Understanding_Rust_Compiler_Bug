
error[E0716]: temporary value dropped while borrowed
 --> src/main.rs:6:18
  |
6 |     let x = bar(&foo());
  |                  ^^^^^ - temporary value is freed at the end of this statement
  |                  |
  |                  creates a temporary which is freed while still in use
7 |     println!("{}", x);
  |                    - borrow later used here
  |
  = note: consider using a `let` binding to create a longer lived value
