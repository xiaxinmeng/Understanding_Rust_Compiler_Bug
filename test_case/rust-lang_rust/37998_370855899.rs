
warning: constant evaluation error: attempt to divide by zero
 --> src/main.rs:1:18
  |
1 | const WHY: i32 = 1 / 0;
  |                  ^^^^^
  |
  = note: #[warn(const_err)] on by default

error[E0080]: constant evaluation error
 --> src/main.rs:1:18
  |
1 | const WHY: i32 = 1 / 0;
  |                  ^^^^^ attempt to divide by zero

error: aborting due to previous error

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
