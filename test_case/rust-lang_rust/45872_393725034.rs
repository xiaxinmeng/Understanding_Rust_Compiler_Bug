
   Compiling playground v0.0.1 (file:///playground)
error[E0641]: cannot cast to a pointer of an unknown kind
 --> src/main.rs:5:14
  |
5 |     let p2 = p1 as *mut _;
  |              ^^^^^^------
  |                    |
  |                    help: consider giving more type information
  |
  = note: The type information given here is insufficient to check whether the pointer cast is valid

error: aborting due to previous error

For more information about this error, try `rustc --explain E0641`.
error: Could not compile `playground`.

To learn more, run the command again with --verbose.
