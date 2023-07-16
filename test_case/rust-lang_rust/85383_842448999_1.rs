
error[E0716]: temporary value dropped while borrowed
  --> src/main.rs:12:13
   |
12 |     let b = a().get();
   |             ^^^      - temporary value is freed at the end of this statement
   |             |
   |             creates a temporary which is freed while still in use
13 |     println!("{}", b.0.to_string());
   |                    --- borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value
