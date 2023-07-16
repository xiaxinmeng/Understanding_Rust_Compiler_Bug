ini
error[E0716]: temporary value dropped while borrowed
 --> src/main.rs:5:18
  |
5 |     let p = bar(&foo());
  |                  ^^^^^ - temporary value is freed at the end of this statement
  |                  |
  |                  creates a temporary which is freed while still in use
6 |     let q = *p;
  |             -- borrow later used here
  |
help: consider using a `let` binding to create a longer lived value
  |
5 ~     let binding = foo();
6 ~     let p = bar(&binding);
  |

For more information about this error, try `rustc --explain E0716`.
