text
error[E0716]: temporary value dropped while borrowed
  --> src/main.rs:11:25
   |
11 |         let mut split = line.unwrap().split(":");
   |                         ^^^^^^^^^^^^^           - temporary value is freed at the end of this statement
   |                         |
   |                         creates a temporary which is freed while still in use
12 | 
13 |         let name = split.next().unwrap().trim();
   |                    ----- borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value
