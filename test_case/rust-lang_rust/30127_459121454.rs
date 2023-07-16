
error[E0716]: temporary value dropped while borrowed
 --> src/main.rs:6:23
  |
6 |     let _desugar2 = &*"a".to_owned().index(..);
  |                       ^^^^^^^^^^^^^^          - temporary value is freed at the end of this statement
  |                       |
  |                       creates a temporary which is freed while still in use
7 |     println!("{:?}", _desugar2);
  |                      --------- borrow later used here
  |
  = note: consider using a `let` binding to create a longer lived value

error: aborting due to previous error
